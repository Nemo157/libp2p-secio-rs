use std::io;
use std::cmp::Ordering;
use std::iter::FromIterator;

use bytes::{Bytes, BytesMut};
use futures::{ Future, Stream, Sink, Poll, Async, AsyncSink };
use identity::{ HostId, PeerId };
use mhash::MultiHash;
use msgio::MsgIo;
use protobuf::{ ProtobufError, Message, parse_from_bytes };
use secstream::{ SecStream };

use crypto::rand;
use crypto::{ HashAlgorithm, CipherAlgorithm, CurveAlgorithm, CurvePrivateKey };
use data::{ Propose, Exchange };

const NONCE_SIZE: usize = 16;

enum Step {
    Propose,
    SendProposal(Bytes),
    SendingProposal,
    ReceivingProposal,
    ProcessingProposal,
    SendExchange(Bytes),
    SendingExchange,
    ReceivingExchange,
    ProcessingExchange(Bytes),
    SendNonce(Bytes),
    SendingNonce,
    ReceivingNonce,
    ProcessingNonce(Bytes),
}

pub struct Handshake<S: MsgIo> {
    transport: Option<S>,
    secstream: Option<SecStream<S>>,
    my_id: HostId,
    their_id: PeerId,
    step: Option<Step>,

    curve: CurveAlgorithm,
    cipher: CipherAlgorithm,
    hash: HashAlgorithm,
    order: Ordering,
    my_nonce: [u8; NONCE_SIZE],
    my_proposal_bytes: Bytes,
    their_proposal_bytes: Bytes,
    my_ephemeral_priv_key: Option<CurvePrivateKey>,
    their_proposal: Propose,
    my_proposal: Propose,
}

fn pbetio(e: ProtobufError) -> io::Error {
    match e {
        ProtobufError::IoError(e) => e,
        ProtobufError::WireError(m) => io::Error::new(io::ErrorKind::Other, m),
        ProtobufError::MessageNotInitialized { message: m } =>
            io::Error::new(io::ErrorKind::Other, m),
    }
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

impl<S: MsgIo> Handshake<S> {
    pub(crate) fn create(transport: S, my_id: HostId, their_id: PeerId) -> Handshake<S> {
        Handshake {
            transport: Some(transport),
            secstream: None,
            my_id: my_id,
            their_id: their_id,
            step: Some(Step::Propose),

            curve: CurveAlgorithm::all()[0],
            cipher: CipherAlgorithm::all()[0],
            hash: HashAlgorithm::all()[0],
            order: Ordering::Equal,
            my_nonce: [0; NONCE_SIZE],
            my_proposal_bytes: Bytes::new(),
            their_proposal_bytes: Bytes::new(),
            my_ephemeral_priv_key: None,
            their_proposal: Propose::new(),
            my_proposal: Propose::new(),
        }
    }
}

impl<S: MsgIo> Future for Handshake<S> {
    type Item = SecStream<S>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.step.take().expect("Only none if something failed") {
                Step::Propose => {
                    // step 1. Propose -- propose cipher suite + send pubkeys + nonce
                    println!("secure handshake start");

                    self.my_nonce = {
                        let mut nonce = [0; NONCE_SIZE];
                        rand::fill(&mut nonce)?;
                        nonce
                    };

                    self.my_proposal = {
                        let mut proposal = Propose::new();
                        proposal.set_rand(self.my_nonce.as_ref().to_owned());
                        proposal.set_pubkey(self.my_id.pub_key().to_protobuf()?);
                        proposal.set_exchanges(CurveAlgorithm::all()[0].to_string());
                        proposal.set_ciphers(CipherAlgorithm::all()[0].to_string());
                        proposal.set_hashes(HashAlgorithm::all()[0].to_string());
                        proposal
                    };

                    println!("Sending proposal (curve, cipher, hash): {:?}", (self.my_proposal.get_exchanges(), self.my_proposal.get_ciphers(), self.my_proposal.get_hashes()));

                    self.my_proposal_bytes = Bytes::from(self.my_proposal.write_to_bytes().map_err(pbetio)?);
                    self.step = Some(Step::SendProposal(self.my_proposal_bytes.clone()));
                }

                Step::SendProposal(bytes) => {
                    match self.transport.as_mut().unwrap().start_send(bytes)? {
                        AsyncSink::Ready => {
                            self.step = Some(Step::SendingProposal);
                        }
                        AsyncSink::NotReady(bytes) => {
                            self.step = Some(Step::SendProposal(bytes));
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::SendingProposal => {
                    match self.transport.as_mut().unwrap().poll_complete()? {
                        Async::Ready(()) => {
                            self.step = Some(Step::ReceivingProposal);
                        }
                        Async::NotReady => {
                            self.step = Some(Step::SendingProposal);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ReceivingProposal => {
                    match self.transport.as_mut().unwrap().poll()? {
                        Async::Ready(Some(bytes)) => {
                            self.their_proposal_bytes = bytes;
                            self.step = Some(Step::ProcessingProposal);
                        }
                        Async::Ready(None) => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"));
                        }
                        Async::NotReady => {
                            self.step = Some(Step::ReceivingProposal);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ProcessingProposal => {
                    self.their_proposal = parse_from_bytes(&self.their_proposal_bytes).map_err(pbetio)?;
                    println!("Received proposal (curve, cipher, hash): {:?}", (self.their_proposal.get_exchanges(), self.their_proposal.get_ciphers(), self.their_proposal.get_hashes()));
                    // // step 1.1 Identify -- get identity from their key
                    self.their_id = {
                        let actual_id = PeerId::from_protobuf(&self.their_proposal.get_pubkey())?;
                        if !actual_id.matches(&self.their_id) {
                            return Err(io::Error::new(io::ErrorKind::Other, format!("public key from actual peer {:?} didn't match provided id {:?}", actual_id, self.their_id)));
                        }
                        actual_id
                    };
                    println!("identified peer as: {:?}", self.their_id);

                    self.order = {
                        let order1 = MultiHash::generate_sha2_256(&Bytes::from(Vec::from_iter(self.their_proposal.get_pubkey().iter().chain(self.my_nonce.iter()).cloned())));
                        let order2 = MultiHash::generate_sha2_256(&Bytes::from(Vec::from_iter(self.my_proposal.get_pubkey().iter().chain(self.their_proposal.get_rand()).cloned())));
                        order1.to_bytes().cmp(&order2.to_bytes())
                    };

                    if self.order == Ordering::Equal {
                        return Err(io::Error::new(io::ErrorKind::Other, "talking to self (same socket. must be reuseport + dialing self)"));
                    }

                    // step 1.2 Selection -- select/agree on best encryption parameters
                    let (curve, cipher, hash) = select(&self.their_proposal, self.order)?;
                    println!("Selected (curve, cipher, hash): {:?}", (curve, cipher, hash));
                    self.curve = curve;
                    self.cipher = cipher;
                    self.hash = hash;

                    // step 2. Exchange -- exchange (signed) ephemeral keys. verify signatures.
                    self.my_ephemeral_priv_key = Some(curve.generate_priv_key()?);
                    let my_ephemeral_pub_key = self.my_ephemeral_priv_key.as_mut().unwrap().pub_key()?;

                    // Gather corpus to sign.
                    let my_corpus = {
                        let mut corpus = BytesMut::new();
                        corpus.extend_from_slice(&self.my_proposal_bytes);
                        corpus.extend_from_slice(&self.their_proposal_bytes);
                        corpus.extend_from_slice(my_ephemeral_pub_key);
                        corpus.freeze()
                    };

                    let my_exchange = {
                        let mut exchange = Exchange::new();
                        exchange.set_epubkey(my_ephemeral_pub_key.to_owned());
                        exchange.set_signature(self.my_id.sign(&my_corpus)?);
                        exchange
                    };

                    println!("Sending exchange");
                    self.step = Some(Step::SendExchange(Bytes::from(my_exchange.write_to_bytes().map_err(pbetio)?)));
                }

                Step::SendExchange(bytes) => {
                    match self.transport.as_mut().unwrap().start_send(bytes)? {
                        AsyncSink::Ready => {
                            self.step = Some(Step::SendingExchange);
                        }
                        AsyncSink::NotReady(bytes) => {
                            self.step = Some(Step::SendExchange(bytes));
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::SendingExchange => {
                    match self.transport.as_mut().unwrap().poll_complete()? {
                        Async::Ready(()) => {
                            self.step = Some(Step::ReceivingExchange);
                        }
                        Async::NotReady => {
                            self.step = Some(Step::SendingExchange);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ReceivingExchange => {
                    match self.transport.as_mut().unwrap().poll()? {
                        Async::Ready(Some(bytes)) => {
                            self.step = Some(Step::ProcessingExchange(bytes));
                        }
                        Async::Ready(None) => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"));
                        }
                        Async::NotReady => {
                            self.step = Some(Step::ReceivingExchange);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ProcessingExchange(bytes) => {
                    let their_exchange: Exchange = parse_from_bytes(&bytes).map_err(pbetio)?;
                    println!("Received exchange");
                    // TODO: This is the wrong format, need to add X9.62 §4.36 unmarshalling to
                    // ring for compatibility with the Go implementation
                    let their_ephemeral_pub_key = their_exchange.get_epubkey();

                    // step 2.1. Verify -- verify their exchange packet is good.
                    let their_corpus = {
                        let mut corpus = BytesMut::new();
                        corpus.extend_from_slice(&self.their_proposal_bytes);
                        corpus.extend_from_slice(&self.my_proposal_bytes);
                        corpus.extend_from_slice(&their_ephemeral_pub_key);
                        corpus
                    };

                    try!(self.their_id.verify(&their_corpus, their_exchange.get_signature()));
                    println!("Verified exchange");

                    // step 2.2. Keys -- generate keys for mac + encryption
                    let algos = self.my_ephemeral_priv_key.take().unwrap().agree_with(their_ephemeral_pub_key, self.hash, self.cipher, self.order == Ordering::Less)?;

                    // step 3. Finish -- send expected message to verify encryption works (send local nonce)
                    self.secstream = Some(SecStream::create(self.their_id.clone(), self.transport.take().unwrap(), algos));
                    self.step = Some(Step::SendNonce(Bytes::from(self.their_proposal.get_rand())));
                }

                Step::SendNonce(bytes) => {
                    match self.secstream.as_mut().unwrap().start_send(bytes)? {
                        AsyncSink::Ready => {
                            self.step = Some(Step::SendingNonce);
                        }
                        AsyncSink::NotReady(bytes) => {
                            self.step = Some(Step::SendNonce(bytes));
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::SendingNonce => {
                    match self.secstream.as_mut().unwrap().poll_complete()? {
                        Async::Ready(()) => {
                            self.step = Some(Step::ReceivingNonce);
                        }
                        Async::NotReady => {
                            self.step = Some(Step::SendingNonce);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ReceivingNonce => {
                    match self.secstream.as_mut().unwrap().poll()? {
                        Async::Ready(Some(bytes)) => {
                            self.step = Some(Step::ProcessingNonce(bytes));
                        }
                        Async::Ready(None) => {
                            return Err(io::Error::new(io::ErrorKind::Other, "Unexpected EOF"));
                        }
                        Async::NotReady => {
                            self.step = Some(Step::ReceivingNonce);
                            return Ok(Async::NotReady);
                        }
                    }
                }

                Step::ProcessingNonce(bytes) => {
                    if self.my_nonce[..] != bytes[..] {
                        println!("my nonce {:?}, they gave {:?}", self.my_nonce, bytes);
                        return Err(io::Error::new(io::ErrorKind::Other, "Nonces did not match"));
                    }

                    return Ok(Async::Ready(self.secstream.take().unwrap()));
                }
            }
        }
    }
}
