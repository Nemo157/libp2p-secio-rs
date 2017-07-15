extern crate bytes;
#[macro_use]
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

use identity::{ HostId, PeerId };
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::FramedParts;

pub use secstream::SecStream;
pub use handshake::Handshake;

pub fn handshake<S: AsyncRead + AsyncWrite>(transport: FramedParts<S>, host: HostId, peer: PeerId) -> Handshake<S> {
    Handshake::create(transport, host, peer)
}
