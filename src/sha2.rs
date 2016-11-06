use ring::{ digest, hmac };

use hash::{ Signer, Verifier };

pub struct Sha2256Signer {
    key: hmac::SigningKey,
}

pub struct Sha2256Verifier {
    key: hmac::VerificationKey,
}

impl Sha2256Signer {
    pub fn new(key: &[u8]) -> Sha2256Signer {
        Sha2256Signer {
            key: hmac::SigningKey::new(&digest::SHA256, key),
        }
    }
}

impl Sha2256Verifier {
    pub fn new(key: &[u8]) -> Sha2256Verifier {
        Sha2256Verifier {
            key: hmac::VerificationKey::new(&digest::SHA256, key),
        }
    }
}

impl Signer for Sha2256Signer {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        hmac::sign(&self.key, data).as_ref().to_vec()
    }

    fn sign_all(&self, datas: &[&[u8]]) -> Vec<u8> {
        let mut ctx = hmac::SigningContext::with_key(&self.key);
        for data in datas {
            ctx.update(data);
        }
        ctx.sign().as_ref().to_vec()
    }
}

impl Verifier for Sha2256Verifier {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), ()> {
        hmac::verify(&self.key, data, signature).map_err(|_| ())
    }

    fn digest_len(&self) -> usize {
        digest::SHA256.output_len
    }
}
