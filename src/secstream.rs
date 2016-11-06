use std::{ io, fmt };
use msgio::{ ReadLpm, WriteLpm };
use identity::{ HostId, PeerId };

use hash::{ Signer, Verifier };
use cipher::{ Encryptor, Decryptor };
use handshake;

pub struct SecStream<S> where S: io::Read + io::Write {
    stream: S,
    encryptor: Box<Encryptor>,
    signer: Box<Signer>,
    decryptor: Box<Decryptor>,
    verifier: Box<Verifier>,
}

pub fn create<S>(stream: S, (encryptor, signer): (Box<Encryptor>, Box<Signer>), (decryptor, verifier): (Box<Decryptor>, Box<Verifier>)) -> SecStream<S> where S: io::Read + io::Write{
    SecStream {
        stream: stream,
        encryptor: encryptor,
        signer: signer,
        decryptor: decryptor,
        verifier: verifier,
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
        let data_len = msg.len() - self.verifier.digest_len();
        let mac = msg.split_off(data_len);
        try!(self.verifier.verify(&msg, &mac).map_err(|_| io::Error::new(io::ErrorKind::Other, "MAC verification failed")));
        let data = try!(self.decryptor.decrypt(&msg).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed")));
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
        let mut data = try!(self.encryptor.encrypt(&buf).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed")));
        let mac = self.signer.sign(&data);
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
