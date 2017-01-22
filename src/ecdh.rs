use std::{ io, mem };

use ring::{ agreement, rand };
use untrusted::Input;

use hash::HashAlgorithm;
use cipher::CipherAlgorithm;
use shared::SharedAlgorithms;

pub struct PrivateKey {
    algo: &'static agreement::Algorithm,
    priv_key: agreement::EphemeralPrivateKey,
    pub_key: Vec<u8>,
}

impl PrivateKey {
    fn new(algo: &'static agreement::Algorithm) -> io::Result<PrivateKey> {
        let rand = rand::SystemRandom::new();
        let priv_key = agreement::EphemeralPrivateKey::generate(algo, &rand)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to compute private key"))?;
        Ok(PrivateKey {
            algo: algo,
            priv_key: priv_key,
            pub_key: Vec::new(),
        })
    }

    pub fn pub_key(&mut self) -> io::Result<&[u8]> {
        if self.pub_key.is_empty() {
            self.pub_key.reserve(self.priv_key.public_key_len());
            self.priv_key.compute_public_key(&mut self.pub_key)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to compute public key"))?;
        }
        Ok(&self.pub_key)
    }

    pub fn agree_with(self, their_pub_key: &[u8], hash: HashAlgorithm, cipher: CipherAlgorithm, swapped: bool) -> io::Result<SharedAlgorithms> {
        let cipher_iv_size = cipher.iv_size();
        let cipher_key_size = cipher.key_size();
        let hash_key_size = hash.key_size();
        let single_size = cipher_iv_size + cipher_key_size + hash_key_size;
        let required_bytes = single_size * 2;

        let bytes = agreement::agree_ephemeral(
            self.priv_key,
            self.algo,
            Input::from(their_pub_key),
            io::Error::new(io::ErrorKind::Other, "agree ephemeral failed"),
            |key| {
                let seed = b"key expansion";

                let signer = hash.signer(key);
                let mut a = signer.sign(seed);

                let mut bytes = Vec::with_capacity(required_bytes);
                while bytes.len() < required_bytes {
                    let b = signer.sign_all(&[a.as_ref(), seed]);
                    bytes.extend_from_slice(b.as_ref());
                    a = signer.sign(a.as_ref());
                }

                bytes.truncate(required_bytes);
                Ok(bytes)
            })?;

        let (first, second) = {
            let (mut first, mut second) = bytes.split_at(single_size);
            if swapped { mem::swap(&mut first, &mut second); }
            (first, second)
        };

        let (encryptor_iv, first) = first.split_at(cipher_iv_size);
        let (decryptor_iv, second) = second.split_at(cipher_iv_size);

        let (encryptor_key, signer_key) = first.split_at(cipher_key_size);
        let (decryptor_key, verifier_key) = second.split_at(cipher_key_size);

        assert_eq!(signer_key.len(), hash_key_size);
        assert_eq!(verifier_key.len(), hash_key_size);

        Ok(SharedAlgorithms {
            signer: hash.signer(signer_key),
            verifier: hash.verifier(verifier_key),
            encryptor: cipher.encryptor(encryptor_key, encryptor_iv),
            decryptor: cipher.decryptor(decryptor_key, decryptor_iv),
        })
    }
}

pub mod p384 {
    use super::PrivateKey;
    use std::io;
    use ring::agreement;

    pub fn generate() -> io::Result<PrivateKey> {
        PrivateKey::new(&agreement::ECDH_P384)
    }
}
