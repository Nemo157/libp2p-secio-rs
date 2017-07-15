use std::io;

use bytes::{Bytes, BytesMut};
use msgio;
use tokio_io::codec::{Decoder, Encoder};

use crypto::hash::{ Signer, Verifier };
use crypto::cipher::{ Encryptor, Decryptor };
use crypto::shared::SharedAlgorithms;

#[derive(Debug)]
pub struct SecStream {
    inner: msgio::LengthPrefixed,
    algos: SharedAlgorithms,
}

impl SecStream {
    pub(crate) fn create(algos: SharedAlgorithms) -> SecStream {
        let inner = msgio::LengthPrefixed(msgio::Prefix::BigEndianU32, msgio::Suffix::None);
        SecStream { inner, algos }
    }

    fn decrypt_msg(&mut self, msg: &[u8]) -> io::Result<Bytes> {
        // MAC is stored at the end of the message.
        // Assume digest algorithm is the same in both directions, should add
        // some way to get the digest size from the VerificationKey.
        let data_len = msg.len() - self.algos.digest_len();
        try!(self.algos.verify(&msg[..data_len], &msg[data_len..]).map_err(|_| io::Error::new(io::ErrorKind::Other, "MAC verification failed")));
        let data = try!(self.algos.decrypt(&msg[..data_len]).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed")));
        Ok(Bytes::from(data))
    }
}

impl Decoder for SecStream {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(if let Some(msg) = self.inner.decode(src)? {
            Some(self.decrypt_msg(&msg)?)
        } else {
            None
        })
    }
}

impl Encoder for SecStream {
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut data = self.algos.encrypt(&item).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        let mac = self.algos.sign(&data);
        data.extend(mac);
        self.inner.encode(Bytes::from(data), dst)
    }
}

impl ::msgio::Codec for SecStream { }
