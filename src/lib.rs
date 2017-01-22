#![feature(pub_restricted)]

#[macro_use]
extern crate futures;
extern crate libp2p_crypto as crypto;
extern crate libp2p_identity as identity;
extern crate mhash;
extern crate protobuf;
extern crate tokio_core;

mod data;
mod handshake;
mod secstream;

use std::io;
use identity::{ HostId, PeerId };
use futures::{ Sink, Stream };
use tokio_core::io::EasyBuf;

pub use secstream::SecStream;
pub use handshake::Handshake;

pub fn handshake<S>(transport: S, host: HostId, peer: PeerId) -> Handshake<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    Handshake::create(transport, host, peer)
}
