#![feature(pub_restricted)]

extern crate libp2p_identity as identity;
extern crate protobuf;
extern crate ring;
extern crate untrusted;
extern crate crypto;
extern crate mhash;
#[macro_use]
extern crate futures;
extern crate tokio_core;

mod aes;
mod cipher;
mod data;
mod handshake;
mod hash;
mod secstream;
mod sha2;

use std::io;
use identity::{ HostId, PeerId };
use futures::{ Sink, Stream };
use tokio_core::io::EasyBuf;

pub use secstream::SecStream;
pub use handshake::Handshake;

pub fn handshake<S>(transport: S, host: HostId, peer: PeerId) -> Handshake<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    Handshake::create(transport, host, peer)
}
