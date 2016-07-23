use std::io;
use identity::{ HostId, PeerId };
use msgio::{ ReadLpm, WriteLpm };
use protobuf::{ ProtobufError, Message, parse_from_bytes };
use ring::{ agreement, digest, hmac, rand };
use untrusted::Input;
use secstream::{ self, SecStream };

use data::{ Propose, Exchange };

// Only supported ECDH curve
const SUPPORTED_CURVE: &'static str = "P-384";
// Only supported Cipher
const SUPPORTED_CIPHER: &'static str = "AES-256";
// Only supported Hash
const SUPPORTED_HASH: &'static str = "SHA256";

const NONCE_SIZE: usize = 16;

fn pbetio(e: ProtobufError) -> io::Error {
    match e {
        ProtobufError::IoError(e) => e,
        ProtobufError::WireError(m) => io::Error::new(io::ErrorKind::Other, m),
        ProtobufError::MessageNotInitialized { message: m } =>
            io::Error::new(io::ErrorKind::Other, m),
    }
}

fn select(proposal: &Propose) -> io::Result<(&'static agreement::Algorithm, &'static str, &'static digest::Algorithm)> {
    // this may be difficult to get to match the go implementation, I think we will need to use the
    // exact same hashing algorithm otherwise we'll decide on a different order to select the
    // parameters in...
    // 
    // Easy workaround for now: we only support one exchange, cipher and hash so we either talk
    // using them or fail to select an implementation.
    if !proposal.get_exchanges().split(',').any(|s| s == SUPPORTED_CURVE) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't not select a common exchange"));
    }
    if !proposal.get_ciphers().split(',').any(|s| s == SUPPORTED_CIPHER) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't not select a common cipher"));
    }
    if !proposal.get_hashes().split(',').any(|s| s == SUPPORTED_HASH) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't not select a common hash"));
    }
    Ok((&agreement::ECDH_P384, SUPPORTED_CIPHER, &digest::SHA256)) // TODO: Return actual (exchange,cipher,hash)
}

pub fn perform<S>(mut stream: S, my_id: &HostId, their_id: &PeerId) -> io::Result<SecStream<S>> where S: io::Read + io::Write {
    let rand = rand::SystemRandom::new();
    println!("secure handshake start");

    // step 1. Propose -- propose cipher suite + send pubkeys + nonce
    let my_nonce = {
        let mut nonce = [0; NONCE_SIZE];
        if rand.fill(&mut nonce).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to generate random bytes"));
        }
        nonce
    };

    let my_proposal = {
        let mut proposal = Propose::new();
        proposal.set_rand(my_nonce.as_ref().to_owned());
        proposal.set_pubkey(try!(my_id.pub_key().to_protobuf()));
        proposal.set_exchanges(SUPPORTED_CURVE.to_owned());
        proposal.set_ciphers(SUPPORTED_CIPHER.to_owned());
        proposal.set_hashes(SUPPORTED_HASH.to_owned());
        proposal
    };

    let my_proposal_bytes = try!(my_proposal.write_to_bytes().map_err(pbetio));
    try!(stream.write_lpm(&my_proposal_bytes));
    println!(
        "Sending proposal (curve, cipher, hash): {:?}",
        (my_proposal.get_exchanges(), my_proposal.get_ciphers(), my_proposal.get_hashes()));

    let their_proposal_bytes = try!(stream.read_lpm());
    let their_proposal: Propose = try!(parse_from_bytes(&their_proposal_bytes).map_err(pbetio));
    println!(
        "Received proposal (curve, cipher, hash): {:?}",
        (their_proposal.get_exchanges(), their_proposal.get_ciphers(), their_proposal.get_hashes()));

    // step 1.1 Identify -- get identity from their key
    let their_id = {
        let actual_id = try!(PeerId::from_protobuf(&their_proposal.get_pubkey()));
        if !actual_id.matches(their_id) {
            return Err(io::Error::new(io::ErrorKind::Other, format!("public key from actual peer {:?} didn't match provided id {:?}", actual_id, their_id)));
        }
        actual_id
    };

    // step 1.2 Selection -- select/agree on best encryption parameters
    let (curve, cipher, hash) = try!(select(&their_proposal));
    println!("Selected (curve, cipher, hash): {:?}", (SUPPORTED_CURVE, cipher, SUPPORTED_HASH));

    // step 2. Exchange -- exchange (signed) ephemeral keys. verify signatures.
    let my_ephemeral_priv_key = try!(agreement::EphemeralPrivateKey::generate(curve, &rand).map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to compute public ephemeral key")));
    let my_ephemeral_pub_key = {
        let mut pub_key = vec![0; my_ephemeral_priv_key.public_key_len()];
        try!(my_ephemeral_priv_key.compute_public_key(&mut pub_key).map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to compute public ephemeral key")));
        // TODO: This is the wrong format, need to add X9.62 §4.36 marshalling
        // to ring for compatibility with the Go implementation
        pub_key
    };

    // Gather corpus to sign.
    let my_corpus = {
        let mut corpus = Vec::new();
        corpus.extend_from_slice(&my_proposal_bytes);
        corpus.extend_from_slice(&their_proposal_bytes);
        corpus.extend_from_slice(&my_ephemeral_pub_key);
        corpus
    };

    let my_exchange = {
        let mut exchange = Exchange::new();
        exchange.set_epubkey(my_ephemeral_pub_key);
        exchange.set_signature(try!(my_id.sign(&rand, &my_corpus)));
        exchange
    };

    let my_exchange_bytes = try!(my_exchange.write_to_bytes().map_err(pbetio));
    println!("Sending exchange");
    try!(stream.write_lpm(&my_exchange_bytes));

    let their_exchange_bytes = try!(stream.read_lpm());
    let their_exchange: Exchange = try!(parse_from_bytes(&their_exchange_bytes).map_err(pbetio));
    println!("Received exchange");
    // TODO: This is the wrong format, need to add X9.62 §4.36 unmarshalling to
    // ring for compatibility with the Go implementation
    let their_ephemeral_pub_key = their_exchange.get_epubkey();

    // step 2.1. Verify -- verify their exchange packet is good.
    let their_corpus = {
        let mut corpus = Vec::new();
        corpus.extend_from_slice(&their_proposal_bytes);
        corpus.extend_from_slice(&my_proposal_bytes);
        corpus.extend_from_slice(&their_ephemeral_pub_key);
        corpus
    };

    try!(their_id.verify(&their_corpus, their_exchange.get_signature()));
    println!("Verified exchange");

    // step 2.2. Keys -- generate keys for mac + encryption

    let (my_shared_keys, their_shared_keys) = try!(agreement::agree_ephemeral(
            my_ephemeral_priv_key,
            curve,
            Input::from(their_ephemeral_pub_key),
            io::Error::new(io::ErrorKind::Other, "agree ephemeral failed"),
            |key| {
                let seed = b"key expansion";
                let (iv_size, cipher_key_size) = match cipher {
                    "AES-256" => (16, 32),
                    _ => unimplemented!(),
                };

                let hmac_key_size = 20;
                let required_bytes = 2 * (iv_size + cipher_key_size + hmac_key_size);
                let mut bytes = Vec::with_capacity(required_bytes);

                let key = hmac::SigningKey::new(hash, key);
                let mut a = {
                    let mut ctx = hmac::SigningContext::with_key(&key);
                    ctx.update(seed);
                    ctx.sign()
                };

                while bytes.len() < required_bytes {
                    let b = {
                        let mut ctx = hmac::SigningContext::with_key(&key);
                        ctx.update(a.as_ref());
                        ctx.update(seed);
                        ctx.sign()
                    };
                    bytes.extend_from_slice(&b.as_ref());
                    a = {
                        let mut ctx = hmac::SigningContext::with_key(&key);
                        ctx.update(a.as_ref());
                        ctx.sign()
                    };
                }

                bytes.truncate(required_bytes);

                let mut first_iv = bytes;
                let mut second_iv = first_iv.split_off(required_bytes / 2);

                let mut first_cipher_key = first_iv.split_off(iv_size);
                let mut second_cipher_key = second_iv.split_off(iv_size);

                let first_mac_key = first_cipher_key.split_off(cipher_key_size);
                let second_mac_key = second_cipher_key.split_off(cipher_key_size);

                let first_shared_key = (first_iv, first_cipher_key, first_mac_key);
                let second_shared_key = (second_iv, second_cipher_key, second_mac_key);

                Ok((first_shared_key, second_shared_key))
            }));

    // TODO: need to maybe swap keys, just try this order for now
    let my_hmac = hmac::SigningKey::new(hash, &my_shared_keys.2);
    let their_hmac = hmac::VerificationKey::new(hash, &their_shared_keys.2);

    // step 3. Finish -- send expected message to verify encryption works (send local nonce)
    let mut secstream = secstream::create(stream, my_hmac, their_hmac);
    try!(secstream.write_lpm(their_proposal.get_rand()));
    let my_nonce_in = try!(secstream.read_lpm());
    if my_nonce[..] != my_nonce_in[..] {
        return Err(io::Error::new(io::ErrorKind::Other, "Nonces did not match"));
    }

    Ok(secstream)
}
