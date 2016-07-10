use std::io::{ self, Write };
use identity::{ HostId, PeerId };
use msgio::{ ReadLpm, WriteLpm };
use protobuf::{ ProtobufError, Message, parse_from_bytes };
use ring::agreement::{ self, EphemeralPrivateKey };
use ring::rand::{ SecureRandom, SystemRandom };

use data::{ Propose, Exchange };

// Only supported ECDH curve
const SUPPORTED_CURVE: &'static str = "P-384";
// Only supported Cipher
const SUPPORTED_CIPHER: &'static str = "AES-256";
// Only supported Hash
const SUPPORTED_HASH: &'static str = "SHA512";

const NONCE_SIZE: usize = 16;

fn pbetio(e: ProtobufError) -> io::Error {
    match e {
        ProtobufError::IoError(e) => e,
        ProtobufError::WireError(m) => io::Error::new(io::ErrorKind::Other, m),
        ProtobufError::MessageNotInitialized { message: m } =>
            io::Error::new(io::ErrorKind::Other, m),
    }
}

pub fn perform<S>(stream: &mut S, my_id: &HostId, their_id: &PeerId) -> io::Result<()> where S: io::Read + io::Write {
    let rand = SystemRandom::new();
    println!("secure handshake start");
    // step 1. Propose -- propose cipher suite + send pubkeys + nonce
    let my_nonce = try!(generate_nonce(&rand));
    let my_proposal = try!(generate_proposal(&my_nonce, &my_id));
    println!("Sending proposal: {:?}", my_proposal);
    try!(send_proposal(stream, &my_proposal));
    let their_proposal = try!(receive_proposal(stream));
    println!("Received proposal: {:?}", their_proposal);
    // step 1.1 Identify -- get identity from their key
    let their_actual_id = try!(PeerId::from_protobuf(&their_proposal.get_pubkey()));
    if !their_actual_id.matches(their_id) {
        return Err(io::Error::new(io::ErrorKind::Other, format!("public key from actual peer {:?} didn't match provided id {:?}", their_actual_id, their_id)));
    }
    // step 1.2 Selection -- select/agree on best encryption parameters
    let (curve, cipher, hash) = try!(select(&their_proposal));
    println!("Selected (curve, cipher, hash): {:?}", (SUPPORTED_CURVE, SUPPORTED_CIPHER, SUPPORTED_HASH));
    // step 2. Exchange -- exchange (signed) ephemeral keys. verify signatures.
    let my_ephemeral_priv_key = try!(generate_ephemeral(&rand, curve));
    let my_exchange = try!(generate_exchange(&my_proposal, &their_proposal, &my_ephemeral_priv_key, &my_id, &rand));
    println!("Sending exchange: {:?}", my_exchange);
    try!(send_exchange(stream, &my_exchange));
    let their_exchange = try!(receive_exchange(stream));
    println!("Received exchange: {:?}", their_exchange);
    Err(io::Error::new(io::ErrorKind::Other, "handshake not fully implemented"))
}

fn generate_nonce(rand: &SecureRandom) -> io::Result<[u8; NONCE_SIZE]> {
    let mut nonce = [0; NONCE_SIZE];
    match rand.fill(&mut nonce) {
        Ok(()) => Ok(nonce),
        Err(()) => Err(io::Error::new(io::ErrorKind::Other, "failed to generate random bytes")),
    }
}

fn generate_proposal(nonce: &[u8; NONCE_SIZE], my_id: &HostId) -> io::Result<Propose> {
    let mut proposal = Propose::new();
    proposal.set_rand(nonce.as_ref().to_owned());
    proposal.set_pubkey(try!(my_id.pub_key().to_protobuf()));
    proposal.set_exchanges(SUPPORTED_CURVE.to_owned());
    proposal.set_ciphers(SUPPORTED_CIPHER.to_owned());
    proposal.set_hashes(SUPPORTED_HASH.to_owned());
    Ok(proposal)
}

fn send_proposal(mut writer: &mut io::Write, proposal: &Propose) -> io::Result<()> {
    let bytes = try!(proposal.write_to_bytes().map_err(pbetio));
    writer.write_lpm(&bytes)
}

fn receive_proposal(mut reader: &mut io::Read) -> io::Result<Propose> {
    let msg = try!(reader.read_lpm());
    parse_from_bytes(&msg).map_err(pbetio)
}

fn select(proposal: &Propose) -> io::Result<(&'static agreement::Algorithm, &'static str, &'static str)> {
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
    Ok((&agreement::ECDH_P384, SUPPORTED_CIPHER, SUPPORTED_HASH)) // TODO: Return actual (exchange,cipher,hash)
}

fn generate_ephemeral(rand: &SecureRandom, curve: &'static agreement::Algorithm) -> io::Result<EphemeralPrivateKey> {
    match EphemeralPrivateKey::generate(curve, rand) {
        Ok(key) => Ok(key),
        Err(()) => Err(io::Error::new(io::ErrorKind::Other, "failed to compute public ephemeral key")),
    }
}

fn generate_exchange(my_proposal: &Propose, their_proposal: &Propose, eph_priv_key: &EphemeralPrivateKey, my_id: &HostId, rand: &SecureRandom) -> io::Result<Exchange> {
    // Gather corpus to sign.
    let mut pub_key = vec![0; eph_priv_key.public_key_len()];
    if eph_priv_key.compute_public_key(&mut pub_key).is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to compute public ephemeral key"));
    }

    let mut size = my_proposal.compute_size() as usize;
    size += their_proposal.compute_size() as usize;
    size += pub_key.len();

    let mut corpus = Vec::with_capacity(size);
    try!(my_proposal.write_to_vec(&mut corpus).map_err(pbetio));
    try!(their_proposal.write_to_vec(&mut corpus).map_err(pbetio));
    try!(corpus.write_all(&pub_key));

    let mut exchange = Exchange::new();
    exchange.set_epubkey(pub_key);
    exchange.set_signature(try!(my_id.sign(rand, &corpus)));
    Ok(exchange)
}

fn send_exchange(mut writer: &mut io::Write, exchange: &Exchange) -> io::Result<()> {
    let bytes = try!(exchange.write_to_bytes().map_err(pbetio));
    writer.write_lpm(&bytes)
}

fn receive_exchange(mut reader: &mut io::Read) -> io::Result<Exchange> {
    let msg = try!(reader.read_lpm());
    parse_from_bytes(&msg).map_err(pbetio)
}

