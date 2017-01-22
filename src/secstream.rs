use std::{ io, fmt };

use futures::{ Sink, Stream, Poll, Async, StartSend };
use tokio_core::io::EasyBuf;

use hash::{ Signer, Verifier };
use cipher::{ Encryptor, Decryptor };
use shared::SharedAlgorithms;

pub struct SecStream<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    transport: S,
    algos: SharedAlgorithms,
}

impl<S> SecStream<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    pub(crate) fn create(transport: S, algos: SharedAlgorithms) -> SecStream<S> {
        SecStream {
            transport: transport,
            algos: algos,
        }
    }

    fn decrypt_msg(&mut self, msg: &[u8]) -> io::Result<Vec<u8>> {
        // MAC is stored at the end of the message.
        // Assume digest algorithm is the same in both directions, should add
        // some way to get the digest size from the VerificationKey.
        let data_len = msg.len() - self.algos.digest_len();
        try!(self.algos.verify(&msg[..data_len], &msg[data_len..]).map_err(|_| io::Error::new(io::ErrorKind::Other, "MAC verification failed")));
        let data = try!(self.algos.decrypt(&msg[..data_len]).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed")));
        Ok(data)
    }
}

impl<S> Stream for SecStream<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(msg) = try_ready!(self.transport.poll()) {
            Ok(Async::Ready(Some(self.decrypt_msg(msg.as_slice())?)))
        } else {
            Ok(Async::Ready(None))
        }
    }
}

impl<S> Sink for SecStream<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> {
    type SinkItem = Vec<u8>;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let mut data = self.algos.encrypt(&item).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        let mac = self.algos.sign(&data);
        data.extend(mac);
        self.transport.start_send(data)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.transport.poll_complete()
    }
}

impl<S> fmt::Debug for SecStream<S> where S: Sink<SinkItem=Vec<u8>, SinkError=io::Error> + Stream<Item=EasyBuf, Error=io::Error> + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SecStream")
            .field("transport", &self.transport)
            .finish()
    }
}
