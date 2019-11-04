use snafu::ResultExt as _;

/// Reads ttyrec frames from a `tokio::io::AsyncRead` instance.
pub struct Reader<R> {
    reader: R,
    parser: crate::parser::Parser,
    read_buf: [u8; 4096],
    done_reading: bool,
}

impl<R: tokio::io::AsyncRead> Reader<R> {
    /// Creates a new `Reader` from a `tokio::io::AsyncRead` instance.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            parser: crate::parser::Parser::new(),
            read_buf: [0; 4096],
            done_reading: false,
        }
    }

    /// Polls to see if a new frame can be read.
    ///
    /// Returns `Ok(Async::Ready(Some(frame)))` if a frame is available,
    /// `Ok(Async::Ready(None))` if the underlying reader is done, and
    /// `Ok(Async::NotReady)` if not enough bytes have been read for a full
    /// frame.
    pub fn poll_read(
        &mut self,
    ) -> futures::Poll<Option<crate::frame::Frame>, crate::error::Error> {
        loop {
            if let Some(frame) = self.parser.next_frame() {
                return Ok(futures::Async::Ready(Some(frame)));
            } else if self.done_reading {
                return Ok(futures::Async::Ready(None));
            }

            let n = futures::try_ready!(self
                .reader
                .poll_read(&mut self.read_buf)
                .context(crate::error::ReadFile));
            if n > 0 {
                self.parser.add_bytes(&self.read_buf[..n]);
            } else {
                self.done_reading = true;
            }
        }
    }

    /// How much the timestamps in this file should be offset by.
    ///
    /// See `Parser::offset`.
    pub fn offset(&self) -> Option<std::time::Duration> {
        self.parser.offset()
    }
}
