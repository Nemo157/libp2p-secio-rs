use hash::{ Signer, Verifier };
use cipher::{ Encryptor, Decryptor };

pub struct SharedAlgorithms {
    pub encryptor: Box<Encryptor>,
    pub decryptor: Box<Decryptor>,
    pub signer: Box<Signer>,
    pub verifier: Box<Verifier>,
}

impl Encryptor for SharedAlgorithms {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()> {
        self.encryptor.encrypt(data)
    }
}

impl Decryptor for SharedAlgorithms {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, ()> {
        self.decryptor.decrypt(data)
    }
}

impl Signer for SharedAlgorithms {
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.signer.sign(data)
    }

    fn sign_all(&self, datas: &[&[u8]]) -> Vec<u8> {
        self.signer.sign_all(datas)
    }
}

impl Verifier for SharedAlgorithms {
    fn verify(&self, data: &[u8], signature: &[u8]) -> Result<(), ()> {
        self.verifier.verify(data, signature)
    }

    fn digest_len(&self) -> usize {
        self.verifier.digest_len()
    }
}
