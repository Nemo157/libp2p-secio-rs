use std::{ fmt, io };

use ecdh;
use hash::HashAlgorithm;
use cipher::CipherAlgorithm;
use shared::SharedAlgorithms;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CurveAlgorithm {
    EcdhP384,
    __NonExhaustive,
}

pub enum CurvePrivateKey {
    ECDH(ecdh::PrivateKey),
    __NonExhaustive,
}

const ALL: &'static [CurveAlgorithm] = &[
    CurveAlgorithm::EcdhP384,
];

impl CurveAlgorithm {
    pub fn all() -> &'static [CurveAlgorithm] { ALL }

    pub fn generate_priv_key(self) -> io::Result<CurvePrivateKey> {
        match self {
            CurveAlgorithm::EcdhP384 => ecdh::p384::generate().map(CurvePrivateKey::ECDH),
            CurveAlgorithm::__NonExhaustive => unreachable!(),
        }
    }
}

impl CurvePrivateKey {
    pub fn pub_key(&mut self) -> io::Result<&[u8]> {
        match *self {
            CurvePrivateKey::ECDH(ref mut priv_key) => priv_key.pub_key(),
            CurvePrivateKey::__NonExhaustive => unreachable!(),
        }
    }

    pub fn agree_with(self, their_pub_key: &[u8], hash: HashAlgorithm, cipher: CipherAlgorithm, swapped: bool) -> io::Result<SharedAlgorithms> {
        match self {
            CurvePrivateKey::ECDH(priv_key) => priv_key.agree_with(their_pub_key, hash, cipher, swapped),
            CurvePrivateKey::__NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for CurveAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            CurveAlgorithm::EcdhP384 => "P-384",
            CurveAlgorithm::__NonExhaustive => unreachable!(),
        })
    }
}
