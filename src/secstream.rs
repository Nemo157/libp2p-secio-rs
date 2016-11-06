use std::{ io, fmt };
use msgio::{ ReadLpm, WriteLpm };
use identity::{ HostId, PeerId };
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer };
use crypto::symmetriccipher::{ Decryptor, Encryptor, SynchronousStreamCipher };

use hash::{ Signer, Verifier };
use handshake;

pub struct SecStream<S> where S: io::Read + io::Write {
    stream: S,
    local_ctr: Box<SynchronousStreamCipher + 'static>,
    local_hmac: Box<Signer>,
    remote_ctr: Box<SynchronousStreamCipher + 'static>,
    remote_hmac: Box<Verifier>,
}

pub fn create<S>(stream: S, (local_ctr, local_hmac): (Box<SynchronousStreamCipher + 'static>, Box<Signer + 'static>), (remote_ctr, remote_hmac): (Box<SynchronousStreamCipher + 'static>, Box<Verifier + 'static>)) -> SecStream<S> where S: io::Read + io::Write{
    SecStream {
        stream: stream,
        local_ctr: local_ctr,
        local_hmac: local_hmac,
        remote_ctr: remote_ctr,
        remote_hmac: remote_hmac,
    }
}

impl<S> SecStream<S> where S: io::Read + io::Write {
    pub fn new(stream: S, host: &HostId, peer: &PeerId) -> io::Result<SecStream<S>> {
        handshake::perform(stream, host, peer)
    }

    fn decrypt_msg(&mut self, mut msg: Vec<u8>) -> io::Result<Vec<u8>> {
        // MAC is stored at the end of the message.
        // Assume digest algorithm is the same in both directions, should add
        // some way to get the digest size from the VerificationKey.
        let data_len = msg.len() - self.remote_hmac.digest_len();
        let mac = msg.split_off(data_len);
        try!(self.remote_hmac.verify(&msg, &mac).map_err(|_| io::Error::new(io::ErrorKind::Other, "MAC verification failed")));
        let mut data = vec![0; data_len];
        try!(self.remote_ctr.decrypt(&mut RefReadBuffer::new(&msg), &mut RefWriteBuffer::new(&mut data), false).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Encryption failed: {:?}", e))));
        Ok(data)
    }
}

impl<S> ReadLpm for SecStream<S> where S: io::Read + io::Write {
    fn read_lpm(&mut self) -> io::Result<Vec<u8>> {
        let msg = try!(self.stream.read_lpm());
        self.decrypt_msg(msg)
    }
    fn try_read_lpm(&mut self) -> io::Result<Option<Vec<u8>>> {
        if let Some(msg) = try!(self.stream.try_read_lpm()) {
            Ok(Some(try!(self.decrypt_msg(msg))))
        } else {
            Ok(None)
        }
    }
}

impl<S> WriteLpm for SecStream<S> where S: io::Read + io::Write {
    fn write_lpm(&mut self, buf: &[u8]) -> io::Result<()> {
        let mut data = vec![0; buf.len()];
        try!(self.local_ctr.encrypt(&mut RefReadBuffer::new(&buf), &mut RefWriteBuffer::new(&mut data), false).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Encryption failed: {:?}", e))));
        let mac = self.local_hmac.sign(&data);
        data.extend(mac);
        self.stream.write_lpm(&data)
    }
}

impl<S> fmt::Debug for SecStream<S> where S: io::Read + io::Write + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SecStream")
            .field("stream", &self.stream)
            .finish()
    }
}
