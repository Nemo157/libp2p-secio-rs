use std::{ io, fmt };
use msgio::{ ReadLpm, WriteLpm };
use identity::{ HostId, PeerId };
use ring::{ hmac };

use handshake;

pub struct SecStream<S> where S: io::Read + io::Write {
    stream: S,
    local_hmac_key: hmac::SigningKey,
    remote_hmac_key: hmac::VerificationKey,
}

pub fn create<S>(stream: S, local_hmac_key: hmac::SigningKey, remote_hmac_key: hmac::VerificationKey) -> SecStream<S> where S: io::Read + io::Write{
    SecStream {
        stream: stream,
        local_hmac_key: local_hmac_key,
        remote_hmac_key: remote_hmac_key,
    }
}

impl<S> SecStream<S> where S: io::Read + io::Write {
    pub fn new(stream: S, host: &HostId, peer: &PeerId) -> io::Result<SecStream<S>> {
        handshake::perform(stream, host, peer)
    }

    fn decrypt_msg(&self, mut msg: Vec<u8>) -> io::Result<Vec<u8>> {
        // MAC is stored at the end of the message.
        // Assume digest algorithm is the same in both directions, should add
        // some way to get the digest size from the VerificationKey.
        let data_len = msg.len() - self.local_hmac_key.digest_algorithm().output_len;
        let mac = msg.split_off(data_len);
        try!(hmac::verify(&self.remote_hmac_key, &msg, &mac).map_err(|_| io::Error::new(io::ErrorKind::Other, "MAC verification failed")));
        // TODO: Decrypt
        Ok(msg)
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
        // TODO: Encrypt
        let mut data = buf.to_owned();
        let mac = {
            let mut ctx = hmac::SigningContext::with_key(&self.local_hmac_key);
            ctx.update(&data);
            ctx.sign()
        };
        data.extend(mac.as_ref());
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
