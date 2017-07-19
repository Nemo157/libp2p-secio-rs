#![feature(conservative_impl_trait)]

extern crate bytes;
extern crate futures;
extern crate libp2p_crypto as crypto;
extern crate libp2p_identity as identity;
extern crate mhash;
extern crate msgio;
extern crate protobuf;
extern crate tokio_io;

mod data;
mod handshake;
mod secstream;

use std::io;

use futures::Future;
use identity::{ HostId, PeerId };
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{FramedParts};

use handshake::Handshake;
pub use secstream::SecStream;

pub fn handshake<S: AsyncRead + AsyncWrite>(transport: FramedParts<S>, host: HostId, peer: PeerId) -> impl Future<Item=(PeerId, SecStream<S>), Error=io::Error> {
    Handshake::new(transport, host, peer)
}
