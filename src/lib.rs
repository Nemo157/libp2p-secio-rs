#![feature(conservative_impl_trait)]
#![feature(generators)]
#![feature(proc_macro)]

extern crate bytes;
extern crate futures_await as futures;
extern crate libp2p_crypto as crypto;
extern crate libp2p_identity as identity;
extern crate mhash;
extern crate msgio;
extern crate protobuf;
extern crate tokio_io;
#[macro_use]
extern crate slog;

mod data;
mod handshake;
mod secstream;

pub use handshake::handshake;
pub use secstream::SecStream;
