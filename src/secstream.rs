use std::cmp;
use std::io::{self, Cursor};

use futures::{Async, AsyncSink, Poll, Stream, Sink};
use bytes::{Buf, Bytes, BytesMut};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed, FramedParts};
use msgio;

use crypto::hash::{ Signer, Verifier };
use crypto::cipher::{ Encryptor, Decryptor };
use crypto::shared::SharedAlgorithms;

#[derive(Debug)]
pub struct SecStream<S> where S: AsyncRead + AsyncWrite {
    done: bool,
    buffer: Cursor<Bytes>,
    inner: Framed<S, SecStreamCodec>,
}

#[derive(Debug)]
struct SecStreamCodec {
    inner: msgio::LengthPrefixed,
    algos: SharedAlgorithms,
}

impl<S> SecStream<S> where S: AsyncRead + AsyncWrite {
    pub(crate) fn new(parts: FramedParts<S>, algos: SharedAlgorithms) -> SecStream<S> {
        SecStream {
            done: false,
            buffer: Cursor::new(Bytes::new()),
            inner: Framed::from_parts(parts, SecStreamCodec::new(algos)),
        }
    }
}

impl SecStreamCodec {
    pub(crate) fn new(algos: SharedAlgorithms) -> SecStreamCodec {
        let inner = msgio::LengthPrefixed(msgio::Prefix::BigEndianU32, msgio::Suffix::None);
        SecStreamCodec { inner, algos }
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

impl<S> io::Read for SecStream<S> where S: AsyncRead + AsyncWrite {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.done {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "stream is closed"));
            }

            if self.buffer.remaining() > 0 {
                let len = cmp::min(self.buffer.remaining(), buf.len());
                self.buffer.copy_to_slice(&mut buf[..len]);
                return Ok(len);
            }

            match self.inner.poll()? {
                Async::Ready(Some(buffer)) => {
                    self.buffer = Cursor::new(buffer);
                }
                Async::Ready(None) => {
                    self.done = true;
                }
                Async::NotReady => {
                    return Err(io::Error::new(io::ErrorKind::WouldBlock, "no data ready"));
                }
            }
        }
    }
}

impl<S> io::Write for SecStream<S> where S: AsyncRead + AsyncWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.start_send(Bytes::from(buf))? {
            AsyncSink::Ready => Ok(buf.len()),
            AsyncSink::NotReady(_) => Err(io::Error::new(io::ErrorKind::WouldBlock, "stream not ready to send")),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner.poll_complete()? {
            Async::Ready(()) => Ok(()),
            Async::NotReady => Err(io::Error::new(io::ErrorKind::WouldBlock, "stream not done sending")),
        }
    }
}

impl<S> AsyncRead for SecStream<S> where S: AsyncRead + AsyncWrite {
}

impl<S> AsyncWrite for SecStream<S> where S: AsyncRead + AsyncWrite {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.inner.close()
    }
}

impl Decoder for SecStreamCodec {
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

impl Encoder for SecStreamCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut data = self.algos.encrypt(&item).map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        let mac = self.algos.sign(&data);
        data.extend(mac);
        self.inner.encode(Bytes::from(data), dst)
    }
}
