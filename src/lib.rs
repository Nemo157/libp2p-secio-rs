extern crate libp2p_identity as identity;
extern crate msgio;
extern crate protobuf;
extern crate ring;
extern crate untrusted;
extern crate crypto;
extern crate mhash;

mod data;
mod secstream;
mod handshake;

pub use secstream::SecStream;
