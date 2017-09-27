use std::io;
use std::cmp::Ordering;
use std::iter::FromIterator;

use bytes::{Bytes, BytesMut};
use futures::{Future, Stream, Sink};
use futures::prelude::{await, async};
use identity::{ HostId, PeerId };
use mhash::MultiHash;
use msgio;
use protobuf::{ ProtobufError, Message, parse_from_bytes };
use secstream::SecStream;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::io::{flush, read_exact, write_all};
use tokio_io::codec::{Framed, FramedParts};
use slog::Logger;

use crypto::rand;
use crypto::{HashAlgorithm, CipherAlgorithm, CurveAlgorithm};
use data::{ Propose, Exchange };

const NONCE_SIZE: usize = 16;

fn pbetio(e: ProtobufError) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

fn select(proposal: &Propose, _order: Ordering) -> io::Result<(CurveAlgorithm, CipherAlgorithm, HashAlgorithm)> {
    // TODO: actually select the algorithms
    if !proposal.get_exchanges().split(',').any(|s| s == CurveAlgorithm::all()[0].to_string()) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't select a common exchange"));
    }
    if !proposal.get_ciphers().split(',').any(|s| s == CipherAlgorithm::all()[0].to_string()) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't select a common cipher"));
    }
    if !proposal.get_hashes().split(',').any(|s| s == HashAlgorithm::all()[0].to_string()) {
        return Err(io::Error::new(io::ErrorKind::Other, "couldn't select a common hash"));
    }
    Ok((CurveAlgorithm::all()[0], CipherAlgorithm::all()[0], HashAlgorithm::all()[0])) // TODO: Return actual (exchange,cipher,hash)
}

#[async]
pub fn handshake<S: AsyncRead + AsyncWrite + 'static>(logger: Logger, transport: FramedParts<S>, host: HostId, peer: PeerId) -> io::Result<(PeerId, SecStream<S>)> {
    let transport = Framed::from_parts(transport, msgio::LengthPrefixed(msgio::Prefix::BigEndianU32, msgio::Suffix::None));

    // step 1. Propose -- propose cipher suite + send pubkeys + nonce
    info!(logger, "secure handshake start");

    let my_nonce = {
        let mut nonce = [0; NONCE_SIZE];
        rand::fill(&mut nonce)?;
        nonce
    };

    let my_proposal = {
        let mut proposal = Propose::new();
        proposal.set_rand(my_nonce.as_ref().to_owned());
        proposal.set_pubkey(host.pub_key().to_protobuf()?);
        proposal.set_exchanges(CurveAlgorithm::all()[0].to_string());
        proposal.set_ciphers(CipherAlgorithm::all()[0].to_string());
        proposal.set_hashes(HashAlgorithm::all()[0].to_string());
        proposal
    };

    info!(logger, "Sending proposal";
        "curves" => my_proposal.get_exchanges(),
        "ciphers" => my_proposal.get_ciphers(),
        "hashes" => my_proposal.get_hashes());

    let my_proposal_bytes = Bytes::from(my_proposal.write_to_bytes().map_err(pbetio)?);

    let transport = await!(transport.send(my_proposal_bytes.clone()))?;

    let (their_proposal_bytes, transport) = await!(transport.into_future().map_err(|(err, _)| err))?;
    let their_proposal_bytes = match their_proposal_bytes {
        Some(bytes) => bytes,
        None => {
            return Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"));
        }
    };

    let mut their_proposal: Propose = parse_from_bytes(&their_proposal_bytes).map_err(pbetio)?;
    info!(logger, "Received proposal";
          "curves" => their_proposal.get_exchanges(),
          "ciphers" => their_proposal.get_ciphers(),
          "hashes" => their_proposal.get_hashes());

    // // step 1.1 Identify -- get identity from their key
    let peer = {
        let actual_id = PeerId::from_protobuf(&their_proposal.get_pubkey())?;
        if let PeerId::Unknown = peer { /* ok */ } else {
            if !actual_id.matches(&peer) {
                return Err(io::Error::new(io::ErrorKind::Other, format!("public key from actual peer {:?} didn't match provided id {:?}", actual_id, peer)));
            }
        }
        actual_id
    };
    info!(logger, "identified peer"; "peer" => ?peer);

    let order = {
        let order1 = MultiHash::generate_sha2_256(&Bytes::from(Vec::from_iter(their_proposal.get_pubkey().iter().chain(my_nonce.iter()).cloned())));
        let order2 = MultiHash::generate_sha2_256(&Bytes::from(Vec::from_iter(my_proposal.get_pubkey().iter().chain(their_proposal.get_rand()).cloned())));
        order1.to_bytes().cmp(&order2.to_bytes())
    };

    if order == Ordering::Equal {
        return Err(io::Error::new(io::ErrorKind::Other, "talking to self (same socket. must be reuseport + dialing self)"));
    }

    // step 1.2 Selection -- select/agree on best encryption parameters
    let (curve, cipher, hash) = select(&their_proposal, order)?;
    info!(logger, "Selected"; "curve" => ?curve, "cipher" => ?cipher, "hash" => ?hash);

    // step 2. Exchange -- exchange (signed) ephemeral keys. verify signatures.
    let mut my_ephemeral_priv_key = curve.generate_priv_key()?;

    // Gather corpus to sign.
    let my_corpus = {
        let mut corpus = BytesMut::new();
        corpus.extend_from_slice(&my_proposal_bytes);
        corpus.extend_from_slice(&their_proposal_bytes);
        corpus.extend_from_slice(my_ephemeral_priv_key.pub_key()?);
        corpus.freeze()
    };

    let my_exchange = {
        let mut exchange = Exchange::new();
        exchange.set_epubkey(my_ephemeral_priv_key.pub_key()?.to_owned());
        exchange.set_signature(host.sign(&my_corpus)?);
        exchange
    };

    info!(logger, "Sending exchange");
    let my_exchange_bytes = Bytes::from(my_exchange.write_to_bytes().map_err(pbetio)?);

    let transport = await!(transport.send(my_exchange_bytes))?;

    let (their_exchange_bytes, transport) = await!(transport.into_future().map_err(|(err, _)| err))?;
    let their_exchange_bytes = match their_exchange_bytes {
        Some(bytes) => bytes,
        None => {
            return Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"));
        }
    };

    let their_exchange: Exchange = parse_from_bytes(&their_exchange_bytes).map_err(pbetio)?;
    info!(logger, "Received exchange");

    // step 2.1. Verify -- verify their exchange packet is good.
    let their_corpus = {
        let mut corpus = BytesMut::new();
        corpus.extend_from_slice(&their_proposal_bytes);
        corpus.extend_from_slice(&my_proposal_bytes);
        corpus.extend_from_slice(&their_exchange.get_epubkey());
        corpus
    };

    try!(peer.verify(&their_corpus, their_exchange.get_signature()));
    info!(logger, "Verified exchange");

    // step 2.2. Keys -- generate keys for mac + encryption
    let algos = my_ephemeral_priv_key.agree_with(their_exchange.get_epubkey(), hash, cipher, order == Ordering::Less)?;

    // step 3. Finish -- send expected message to verify encryption works (send local nonce)
    let parts = transport.into_parts();
    let secstream = SecStream::new(logger.clone(), parts, algos);
    let nonce = their_proposal.take_rand();
    let (secstream, _) = await!(write_all(secstream, nonce))?;
    let secstream = await!(flush(secstream))?;
    let (secstream, bytes) = await!(read_exact(secstream, [0; NONCE_SIZE]))?;
    if my_nonce[..] != bytes[..] {
        info!(logger, "my nonce {:?}, they gave {:?}", my_nonce, bytes);
        return Err(io::Error::new(io::ErrorKind::Other, "Nonces did not match"));
    }

    return Ok((peer.clone(), secstream));
}
