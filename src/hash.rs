use std::fmt;

use sha2;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha2256,
    __NonExhaustive,
}

pub trait Signer {
    fn sign(&self, data: &[u8]) -> Vec<u8>;
    fn sign_all(&self, datas: &[&[u8]]) -> Vec<u8>;
}

pub trait Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), ()>;
    fn digest_len(&self) -> usize;
}

const ALL: &'static [HashAlgorithm] = &[
    HashAlgorithm::Sha2256,
];

impl HashAlgorithm {
    pub fn all() -> &'static [HashAlgorithm] { ALL }

    pub fn key_size(self) -> usize {
        match self {
            HashAlgorithm::Sha2256 => 20,
            HashAlgorithm::__NonExhaustive => unreachable!(),
        }
    }

    pub fn signer(self, key: &[u8]) -> Box<Signer> {
        match self {
            HashAlgorithm::Sha2256 => Box::new(sha2::Sha2256Signer::new(key)),
            HashAlgorithm::__NonExhaustive => unreachable!(),
        }
    }

    pub fn verifier(self, key: &[u8]) -> Box<Verifier> {
        match self {
            HashAlgorithm::Sha2256 => Box::new(sha2::Sha2256Verifier::new(key)),
            HashAlgorithm::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            HashAlgorithm::Sha2256 => "SHA256",
            HashAlgorithm::__NonExhaustive => unreachable!(),
        })
    }
}
