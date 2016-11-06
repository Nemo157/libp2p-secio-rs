use crypto::aes::{ self, KeySize };
use crypto::buffer::{ RefReadBuffer, RefWriteBuffer };
use crypto::symmetriccipher::{
    Decryptor as CryptoDecryptor,
    Encryptor as CryptoEncryptor,
    SynchronousStreamCipher,
};

use cipher::{ Encryptor, Decryptor };

pub struct Aes256Encryptor {
    cipher: Box<SynchronousStreamCipher + 'static>,
}

pub struct Aes256Decryptor {
    cipher: Box<SynchronousStreamCipher + 'static>,
}

impl Aes256Encryptor {
    pub fn new(key: &[u8], iv: &[u8]) -> Aes256Encryptor {
        Aes256Encryptor {
            cipher: aes::ctr(KeySize::KeySize256, key, iv),
        }
    }
}

impl Aes256Decryptor {
    pub fn new(key: &[u8], iv: &[u8]) -> Aes256Decryptor {
        Aes256Decryptor {
            cipher: aes::ctr(KeySize::KeySize256, key, iv),
        }
    }
}

impl Encryptor for Aes256Encryptor {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()> {
        let mut output = vec![0; data.len()];
        {
            let data = &mut RefReadBuffer::new(&data);
            let output = &mut RefWriteBuffer::new(&mut output);
            try!(self.cipher.encrypt(data, output, false).map_err(|_| ()));
        }
        Ok(output)
    }
}

impl Decryptor for Aes256Decryptor {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()> {
        let mut output = vec![0; data.len()];
        {
            let data = &mut RefReadBuffer::new(&data);
            let output = &mut RefWriteBuffer::new(&mut output);
            try!(self.cipher.decrypt(data, output, false).map_err(|_| ()));
        }
        Ok(output)
    }
}
