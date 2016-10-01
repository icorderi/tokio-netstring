//! Frame a stream of bytes based on [netstring]
//!
//! Many protocols delimit their frames by prefacing frame data with a
//! frame head that specifies the length of the frame. The
//! `tokio-netstring` crate provides utilities for handling the netstring
//! based framing. This allows the consumer to work with entire frames
//! without having to worry about buffering or other framing logic.
//!
//! [netstring]: https://en.wikipedia.org/wiki/Netstring
//!
//! # Getting started
//!
//! If implementing a protocol from scratch, using netstring delimited framing
//! is an easy way to get started. [`Framed::new()`] will adapt a
//! full-duplex byte stream with a length delimited framer using default
//! configuration values.
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! #
//! use tokio_io::{AsyncRead, AsyncWrite};
//! use tokio_netstring as netstring;
//!
//! fn bind_transport<T: AsyncRead + AsyncWrite>(io: T)
//!     -> netstring::Framed<T>
//! {
//!     netstring::Framed::new(io)
//! }
//! #
//! # fn main() {}
//! ```
//!
//! The returned transport implements `Sink + Stream` for `BytesMut`. It
//! encodes the frame using the netstring format:
//!
//! ```text
//! +-------------+---+---------------------+---+
//! |  len: utf8  |':'|   frame payload     |','|
//! +-------------+---+---------------------+---+
//! ```
//!
//! Specifically, given the following:
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! # extern crate bytes;
//! # extern crate futures;
//! #
//! use tokio_io::{AsyncRead, AsyncWrite};
//! use tokio_netstring as netstring;
//! use bytes::BytesMut;
//! use futures::{Sink, Future};
//!
//! fn write_frame<T: AsyncRead + AsyncWrite>(io: T) {
//!     let mut transport = netstring::Framed::new(io);
//!     let frame = BytesMut::from("hello world");
//!
//!     transport.send(frame).wait().unwrap();
//! }
//! #
//! # pub fn main() {}
//! ```
//!
//! The encoded frame will look like this:
//!
//! ```text
//! +-- len: utf8 --+-+--- data ----+-+
//! |       11      |:| hello world |,|
//! +---------------+-+-------------+-+
//! ```
//!
//! # Decoding
//!
//! [`FramedRead`] adapts an [`AsyncRead`] into a `Stream` of [`BytesMut`],
//! such that each yielded [`BytesMut`] value contains the contents of an
//! entire frame. There are many configuration paramaters enabling
//! [`FrameRead`] to handle a wide range of protocols. Here are some
//! examples that will cover the various options at a high level.
//!
//! ## Example 1
//!
//! The following will parse a length field at offset 0, including the
//! frame head in the yielded `BytesMut`.
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! #
//! # use tokio_io::AsyncRead;
//! # use tokio_netstring as netstring;
//! # fn bind_read<T: AsyncRead>(io: T) {
//! netstring::Builder::new()
//!     .length_field_offset(0) // default value
//!     .strip_frame(false) // Do not strip frame header
//!     .new_read(io);
//! # }
//! # fn main() {}
//! ```
//!
//! The following frame will be decoded as such:
//!
//! ```text
//!           INPUT                               DECODED
//! +-- len --+-+--- Payload ---+-+     +-- len --+-+--- Payload ---+-+
//! |   11    |:|  Hello world  |,| --> |   11    |:|  Hello world  |,|
//! +---------+-+---------------+-+     +---------+-+---------------+-+
//! ```
//!
//! The value of the length field is `"11"` which represents the length
//! of the payload, `hello world`. [`FramedRead`] assumes that
//! the length field represents the number of bytes that **follows** the
//! `':'`. Thus, the entire frame has a length of 15: 2 bytes for the
//! length, 1 byte for the `':'`, 11 bytes for the payload and 1 byte for the `','`.
//!
//! ## Example 2
//!
//! The following will parse a length field at offset 0, omitting the
//! frame head in the yielded `BytesMut`.
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! #
//! # use tokio_io::AsyncRead;
//! # use tokio_netstring as netstring;
//! # fn bind_read<T: AsyncRead>(io: T) {
//! netstring::Builder::new()
//!     .length_field_offset(0) // default value
//!     // `strip_frame` is not needed, the default is to skip
//!     .new_read(io);
//! # }
//! # fn main() {}
//! ```
//!
//! The following frame will be decoded as such:
//!
//! ```text
//!           INPUT                          DECODED
//! +-- len --+-+--- Payload ---+-+     +--- Payload ---+
//! |   11    |:|  Hello world  |,| --> |  Hello world  |
//! +---------+-+---------------+-+     +---------------+
//! ```
//!
//! This is similar to the first example, the only difference is that the
//! frame head is **not** included in the yielded `BytesMut` value.
//!
//! ## Example 3
//!
//! The following will parse a length field at offset 1 of a 4 byte
//! frame head. The first byte and the length will be omitted from the
//! yielded `BytesMut`.
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! #
//! # use tokio_io::AsyncRead;
//! # use tokio_netstring as netstring;
//! # fn bind_read<T: AsyncRead>(io: T) {
//! netstring::Builder::new()
//!     .length_field_offset(1) // length of hdr1
//!     // `strip_frame` is not needed, the default is to skip
//!     .new_read(io);
//! # }
//! # fn main() {}
//! ```
//!
//! The following frame will be decoded as such:
//!
//! ```text
//!                    INPUT                          DECODED
//! +- hdr1 -+-- len --+-+--- Payload ---+-+     +--- Payload ---+
//! |  \xFF  |   11    |:|  Hello world  |,| --> |  Hello world  |
//! +--------+---------+-+---------------+-+     +---------------+
//! ```
//!
//! The length field is situated in the middle of the frame head. In this
//! case, the first byte in the frame head could be a version or some other
//! identifier that is not needed for processing.
//!
//! `length_field_offset` indicates how many bytes to skip before starting
//! to read the length field.
//!
//! # Encoding
//!
//! [`FramedWrite`] adapts an [`AsyncWrite`] into a `Sink` of [`BytesMut`],
//! such that each submitted [`BytesMut`] is prefaced by a netstring length
//! and followed by a `','`.
//! There are fewer configuration options than [`FramedRead`]. Given
//! protocols that have more complex frame heads, an encoder should probably
//! be written by hand using [`Encoder`].
//!
//! Here is a simple example, given a `FramedWrite` with the following
//! configuration:
//!
//! ```
//! # extern crate tokio_io;
//! # extern crate tokio_netstring;
//! # extern crate bytes;
//! #
//! # use tokio_io::AsyncWrite;
//! # use tokio_netstring as netstring;
//! # use bytes::BytesMut;
//! # fn write_frame<T: AsyncWrite>(io: T) {
//! # let _: netstring::FramedWrite<T, BytesMut> =
//! netstring::Builder::new()
//!     .new_write(io);
//! # }
//! # pub fn main() {}
//! ```
//!
//! A payload of `hello world` will be encoded as:
//!
//! ```text
//! +-- len: utf8 --+-+--- data ----+-+
//! |       11      |:| hello world |,|
//! +---------------+-+-------------+-+
//! ```
//!
//! [`FramedRead`]: struct.FramedRead.html
//! [`FramedWrite`]: struct.FramedWrite.html
//! [`AsyncRead`]: ../../trait.AsyncRead.html
//! [`AsyncWrite`]: ../../trait.AsyncWrite.html
//! [`Encoder`]: ../trait.Encoder.html
//! [`BytesMut`]: https://docs.rs/bytes/~0.4/bytes/struct.BytesMut.html

extern crate bytes;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate tokio_io;

use tokio_io::{codec, AsyncRead, AsyncWrite};

use bytes::{Buf, BufMut, BytesMut, IntoBuf};
use bytes::buf::Chain;

use futures::{Async, AsyncSink, Stream, Sink, StartSend, Poll};

use std::fmt;
use std::io::{self, Cursor};

// The following empty netstring `0:,` is the smallest one
const MINIMUM_NETSTRING: usize = 3;

const NETSTRING_TAIL: &'static [u8] = &[b','];

/// Configure netstring delimited `FramedRead`, `FramedWrite`, and `Framed` values.
///
/// `Builder` enables constructing configured netstring delimited framers. Note
/// that not all configuration settings apply to both encoding and decoding. See
/// the documentation for specific methods for more detail.
#[derive(Debug, Clone, Copy)]
pub struct Builder {
    // Maximum frame length
    max_frame_len: usize,

    // Number of bytes in the header before the length field
    length_field_offset: usize,

    // Remove the length, ':' and trailing ','
    strip_frame: bool,
}

/// Adapts a byte stream into a unified `Stream` and `Sink` that works over
/// entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
pub struct Framed<T, B: IntoBuf = BytesMut> {
    inner: FramedRead<FramedWrite<T, B>>,
}

/// Adapts a byte stream to a `Stream` yielding entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
#[derive(Debug)]
pub struct FramedRead<T> {
    inner: codec::FramedRead<T, Decoder>,
}

#[derive(Debug)]
struct Decoder {
    // Configuration values
    builder: Builder,

    // Read state
    state: DecodeState,
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Head,
    Data(usize),
}

/// Adapts a byte stream to a `Sink` accepting entire frame values.
///
/// See [module level] documentation for more detail.
///
/// [module level]: index.html
pub struct FramedWrite<T, B: IntoBuf = BytesMut> {
    // I/O type
    inner: T,

    // Configuration values
    builder: Builder,

    // Current frame being written
    frame: Option<Chain<Chain<Cursor<BytesMut>, B::Buf>, Cursor<&'static [u8]>>>,
}

// ===== impl Framed =====

impl<T: AsyncRead + AsyncWrite, B: IntoBuf> Framed<T, B> {
    /// Creates a new `Framed` with default configuration values.
    pub fn new(inner: T) -> Framed<T, B> {
        Builder::new().new_framed(inner)
    }
}

impl<T, B: IntoBuf> Framed<T, B> {
    /// Returns a reference to the underlying I/O stream wrapped by `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref().get_ref()
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut().get_mut()
    }

    /// Consumes the `Framed`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner().into_inner()
    }
}

impl<T: AsyncRead, B: IntoBuf> Stream for Framed<T, B> {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<BytesMut>, io::Error> {
        self.inner.poll()
    }
}

impl<T: AsyncWrite, B: IntoBuf> Sink for Framed<T, B> {
    type SinkItem = B;
    type SinkError = io::Error;

    fn start_send(&mut self, item: B) -> StartSend<B, io::Error> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        self.inner.close()
    }
}

impl<T, B: IntoBuf> fmt::Debug for Framed<T, B>
    where T: fmt::Debug,
          B::Buf: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Framed")
            .field("inner", &self.inner)
            .finish()
    }
}

// ===== impl FramedRead =====

impl<T: AsyncRead> FramedRead<T> {
    /// Creates a new `FramedRead` with default configuration values.
    pub fn new(inner: T) -> FramedRead<T> {
        Builder::new().new_read(inner)
    }
}

impl<T> FramedRead<T> {
    /// Returns a reference to the underlying I/O stream wrapped by `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `FramedRead`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `FramedRead`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: AsyncRead> Stream for FramedRead<T> {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<BytesMut>, io::Error> {
        self.inner.poll()
    }
}

impl<T: Sink> Sink for FramedRead<T> {
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: T::SinkItem) -> StartSend<T::SinkItem, T::SinkError> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.inner.close()
    }
}

impl<T: io::Write> io::Write for FramedRead<T> {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        self.inner.get_mut().write(src)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.get_mut().flush()
    }
}

impl<T: AsyncWrite> AsyncWrite for FramedRead<T> {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.inner.get_mut().shutdown()
    }

    fn write_buf<B: Buf>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        self.inner.get_mut().write_buf(buf)
    }
}

// ===== impl Decoder ======

impl Decoder {
    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        if src.len() < self.builder.length_field_offset + MINIMUM_NETSTRING {
            // Not enough data
            return Ok(None);
        }

        let (n, i) = {
            let mut src = Cursor::new(&mut *src);

            // Skip the required bytes
            src.advance(self.builder.length_field_offset);

            // Find the next `:` delimiting the end of the length
            if let Some(i) = src.bytes().iter().position(|b| *b == b':') {
                // Parse length
                let n: u64 = match String::from_utf8(src.bytes()[..i].to_vec()) {
                    Ok(s) => {
                        s.parse()
                            .map_err(|_| {
                                io::Error::new(io::ErrorKind::InvalidData, "Could not parse length")
                            })?
                    }
                    Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
                };

                if n > self.builder.max_frame_len as u64 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "frame size too big"));
                }

                // The check above ensures there is no overflow
                (n as usize, i)
            } else {
                return Ok(None);
            }
        };

        if self.builder.strip_frame {
            // | length_field_offset | netstring |':'| payload
            let num_skip = self.builder.length_field_offset + i + 1;
            let _ = src.split_to(num_skip);
        }

        // Ensure that the buffer has enough space to read the incoming
        // payload
        // Note: there is a ',' after the payload
        src.reserve(n + 1);

        return Ok(Some(n));
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        // At this point, the buffer has already had the required capacity
        // reserved. All there is to do is read.
        // Note: The `+1` is for the ',' after the payload
        if src.len() < n + 1 {
            return Ok(None);
        }

        if self.builder.strip_frame {
            // Get the content
            let content = src.split_to(n);

            // Remove the ',' at the end
            let _ = src.split_to(1);

            Ok(Some(content))
        } else {
            Ok(Some(src.take()))
        }
    }
}

impl codec::Decoder for Decoder {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        let n = match self.state {
            DecodeState::Head => {
                match try!(self.decode_head(src)) {
                    Some(n) => {
                        self.state = DecodeState::Data(n);
                        n
                    }
                    None => return Ok(None),
                }
            }
            DecodeState::Data(n) => n,
        };

        match try!(self.decode_data(n, src)) {
            Some(data) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                src.reserve(self.builder.length_field_offset + MINIMUM_NETSTRING);

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}

// ===== impl FramedWrite =====

impl<T: AsyncWrite, B: IntoBuf> FramedWrite<T, B> {
    /// Creates a new `FramedWrite` with default configuration values.
    pub fn new(inner: T) -> FramedWrite<T, B> {
        Builder::new().new_write(inner)
    }
}

impl<T, B: IntoBuf> FramedWrite<T, B> {
    /// Returns a reference to the underlying I/O stream wrapped by
    /// `FramedWrite`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the underlying I/O stream wrapped by
    /// `FramedWrite`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes the `FramedWrite`, returning its underlying I/O stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: AsyncWrite, B: IntoBuf> FramedWrite<T, B> {
    // If there is a buffered frame, try to write it to `T`
    fn do_write(&mut self) -> Poll<(), io::Error> {
        if self.frame.is_none() {
            return Ok(Async::Ready(()));
        }

        loop {
            let frame = self.frame.as_mut().unwrap();
            try_ready!(self.inner.write_buf(frame));

            if !frame.has_remaining() {
                break;
            }
        }

        self.frame = None;

        Ok(Async::Ready(()))
    }

    fn set_frame(&mut self, buf: B::Buf) -> io::Result<()> {
        let mut head = BytesMut::with_capacity(8);
        let n = buf.remaining();

        if n > self.builder.max_frame_len {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "frame too big"));
        }

        let netstring = format!("{}:", n);
        head.put_slice(netstring.as_bytes());

        debug_assert!(self.frame.is_none());

        self.frame = Some(head.into_buf().chain(buf).chain(NETSTRING_TAIL));

        Ok(())
    }
}

impl<T: AsyncWrite, B: IntoBuf> Sink for FramedWrite<T, B> {
    type SinkItem = B;
    type SinkError = io::Error;

    fn start_send(&mut self, item: B) -> StartSend<B, io::Error> {
        if !try!(self.do_write()).is_ready() {
            return Ok(AsyncSink::NotReady(item));
        }

        try!(self.set_frame(item.into_buf()));

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        // Write any buffered frame to T
        try_ready!(self.do_write());

        // Try flushing the underlying IO
        try_nb!(self.inner.flush());

        return Ok(Async::Ready(()));
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        try_ready!(self.poll_complete());
        self.inner.shutdown()
    }
}

impl<T: Stream, B: IntoBuf> Stream for FramedWrite<T, B> {
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<T::Item>, T::Error> {
        self.inner.poll()
    }
}

impl<T: io::Read, B: IntoBuf> io::Read for FramedWrite<T, B> {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        self.get_mut().read(dst)
    }
}

impl<T: AsyncRead, U: IntoBuf> AsyncRead for FramedWrite<T, U> {
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        self.get_mut().read_buf(buf)
    }

    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.get_ref().prepare_uninitialized_buffer(buf)
    }
}

impl<T, B: IntoBuf> fmt::Debug for FramedWrite<T, B>
    where T: fmt::Debug,
          B::Buf: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FramedWrite")
            .field("inner", &self.inner)
            .field("builder", &self.builder)
            .field("frame", &self.frame)
            .finish()
    }
}

// ===== impl Builder =====

impl Builder {
    /// Creates a new length delimited framer builder with default configuration
    /// values.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// #
    /// # use tokio_io::AsyncRead;
    /// use tokio_netstring::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(0)
    ///     .strip_frame(true)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new() -> Builder {
        Builder {
            // Default max frame length of 32MB
            max_frame_len: 32 * 1_024 * 1_024,

            // Default to the header field being at the start of the header.
            length_field_offset: 0,

            // Default to strip the frame.
            strip_frame: true,
        }
    }

    /// Sets the max frame length
    ///
    /// This configuration option applies to both encoding and decoding. The
    /// default value is 8MB.
    ///
    /// When decoding, the length field read from the byte stream is checked
    /// against this setting **before** any adjustments are applied. When
    /// encoding, the length of the submitted payload is checked against this
    /// setting.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// #
    /// # use tokio_io::AsyncRead;
    /// use tokio_netstring::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .max_frame_length(8 * 1024)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn max_frame_length(&mut self, val: usize) -> &mut Self {
        self.max_frame_len = val;
        self
    }

    /// Sets the number of bytes in the header before the length field
    ///
    /// This configuration option only applies to decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// #
    /// # use tokio_io::AsyncRead;
    /// use tokio_netstring::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(1)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn length_field_offset(&mut self, val: usize) -> &mut Self {
        self.length_field_offset = val;
        self
    }

    /// Sets wether or not to streip the frame and leave only the payload
    ///
    /// Default value is `true`
    ///
    /// This configuration option only applies to decoding
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// #
    /// # use tokio_io::AsyncRead;
    /// use tokio_netstring::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .strip_frame(false)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn strip_frame(&mut self, val: bool) -> &mut Self {
        self.strip_frame = val;
        self
    }

    /// Create a configured length delimited `FramedRead`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// #
    /// # use tokio_io::AsyncRead;
    /// use tokio_netstring::Builder;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// Builder::new()
    ///     .length_field_offset(0)
    ///     .strip_frame(false)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_read<T>(&self, upstream: T) -> FramedRead<T>
        where T: AsyncRead
    {
        FramedRead {
            inner: codec::FramedRead::new(upstream,
                                          Decoder {
                                              builder: *self,
                                              state: DecodeState::Head,
                                          }),
        }
    }

    /// Create a configured length delimited `FramedWrite`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// # extern crate bytes;
    /// #
    /// # use tokio_io::AsyncWrite;
    /// # use tokio_netstring as netstring;
    /// # use bytes::BytesMut;
    /// # fn write_frame<T: AsyncWrite>(io: T) {
    /// # let _: netstring::FramedWrite<T, BytesMut> =
    /// netstring::Builder::new()
    ///     .new_write(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_write<T, B>(&self, inner: T) -> FramedWrite<T, B>
        where T: AsyncWrite,
              B: IntoBuf
    {
        FramedWrite {
            inner: inner,
            builder: *self,
            frame: None,
        }
    }

    /// Create a configured netstring delimited `Framed`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tokio_io;
    /// # extern crate tokio_netstring;
    /// # extern crate bytes;
    /// #
    /// # use tokio_io::{AsyncRead, AsyncWrite};
    /// # use tokio_netstring as netstring;
    /// # use bytes::BytesMut;
    /// # fn write_frame<T: AsyncRead + AsyncWrite>(io: T) {
    /// # let _: netstring::Framed<T, BytesMut> =
    /// netstring::Builder::new()
    ///     .new_framed(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_framed<T, B>(&self, inner: T) -> Framed<T, B>
        where T: AsyncRead + AsyncWrite,
              B: IntoBuf
    {
        let inner = self.new_read(self.new_write(inner));
        Framed { inner: inner }
    }
}
