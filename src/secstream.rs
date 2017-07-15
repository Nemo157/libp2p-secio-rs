use std::{ io, fmt };

use bytes::Bytes;
use futures::{ Sink, Stream, Poll, Async, StartSend };
use msgio::MsgIo;
use tokio_io::{AsyncRead, AsyncWrite};

use identity::PeerId;
use crypto::hash::{ Signer, Verifier };
use crypto::cipher::{ Encryptor, Decryptor };
use crypto::shared::SharedAlgorithms;

type Framed<S> = ::tokio_io::codec::Framed<S, ::msgio::LengthPrefixed>;

pub struct SecStream<S: AsyncRead + AsyncWrite> {
    peer: PeerId,
    transport: Framed<S>,
    algos: SharedAlgorithms,
}

impl<S: AsyncRead + AsyncWrite> SecStream<S> {
    pub(crate) fn create(peer: PeerId, transport: Framed<S>, algos: SharedAlgorithms) -> SecStream<S> {
        SecStream { peer, transport, algos }
    }

    pub fn peer(&self) -> &PeerId {
        &self.peer
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

impl<S: AsyncRead + AsyncWrite> Stream for SecStream<S> {
    type Item = Bytes;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(msg) = try_ready!(self.transport.poll()) {
            Ok(Async::Ready(Some(self.decrypt_msg(&msg)?)))
        } else {
            Ok(Async::Ready(None))
        }
    }
}

impl<S: AsyncRead + AsyncWrite> Sink for SecStream<S> {
    type SinkItem = Bytes;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let mut data = self.algos.encrypt(&item).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        let mac = self.algos.sign(&data);
        data.extend(mac);
        self.transport.start_send(Bytes::from(data))
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.transport.poll_complete()
    }
}

impl<S: AsyncRead + AsyncWrite> MsgIo for SecStream<S> { }

impl<S: AsyncRead + AsyncWrite + fmt::Debug> fmt::Debug for SecStream<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SecStream")
            .field("transport", &self.transport)
            .finish()
    }
}
