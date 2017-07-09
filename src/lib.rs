#![feature(pub_restricted)]

#[macro_use]
extern crate futures;
extern crate libp2p_crypto as crypto;
extern crate libp2p_identity as identity;
extern crate mhash;
extern crate protobuf;
extern crate msgio;

mod data;
mod handshake;
mod secstream;

use identity::{ HostId, PeerId };
use msgio::MsgIo;

pub use secstream::SecStream;
pub use handshake::Handshake;

pub fn handshake<S: MsgIo>(transport: S, host: HostId, peer: PeerId) -> Handshake<S> {
    Handshake::create(transport, host, peer)
}
