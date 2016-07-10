use std::io;
use identity::{ HostId, PeerId };

use handshake;

#[derive(Debug)]
pub struct SecStream<S> where S: io::Read + io::Write {
    stream: S,
    stuff: (), // ephemeral key etc.
}

impl<S> SecStream<S> where S: io::Read + io::Write {
    pub fn new(mut stream: S, host: &HostId, peer: &PeerId) -> io::Result<SecStream<S>> {
        let stuff = try!(handshake::perform(&mut stream, host, peer));
        Ok(SecStream {
            stream: stream,
            stuff: stuff,
        })
    }
}
