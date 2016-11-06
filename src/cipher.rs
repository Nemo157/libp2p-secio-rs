use std::fmt;

use aes;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CipherAlgorithm {
    Aes256,
    __NonExhaustive,
}

pub trait Encryptor {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()>;
}

pub trait Decryptor {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()>;
}

const ALL: &'static [CipherAlgorithm] = &[
    CipherAlgorithm::Aes256,
];

impl CipherAlgorithm {
    pub fn all() -> &'static [CipherAlgorithm] { ALL }

    pub fn key_size(self) -> usize {
        match self {
            CipherAlgorithm::Aes256 => 32,
            CipherAlgorithm::__NonExhaustive => unreachable!(),
        }
    }

    pub fn iv_size(self) -> usize {
        match self {
            CipherAlgorithm::Aes256 => 16,
            CipherAlgorithm::__NonExhaustive => unreachable!(),
        }
    }

    pub fn encryptor(self, key: &[u8], iv: &[u8]) -> Box<Encryptor> {
        match self {
            CipherAlgorithm::Aes256 => Box::new(aes::Aes256Encryptor::new(key, iv)),
            CipherAlgorithm::__NonExhaustive => unreachable!(),
        }
    }

    pub fn decryptor(self, key: &[u8], iv: &[u8]) -> Box<Decryptor> {
        match self {
            CipherAlgorithm::Aes256 => Box::new(aes::Aes256Decryptor::new(key, iv)),
            CipherAlgorithm::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for CipherAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CipherAlgorithm::Aes256 => "AES-256",
            CipherAlgorithm::__NonExhaustive => unreachable!(),
        })
    }
}
