use snafu::ResultExt as _;

/// Writes ttyrec frames to a `tokio::io::AsyncWrite` instance.
pub struct Writer<W> {
    writer: W,
    creator: crate::creator::Creator,
    to_write: std::collections::VecDeque<u8>,
}

impl<W: tokio::io::AsyncWrite> Writer<W> {
    /// Creates a new `Writer` from a `tokio::io::AsyncWrite` instance.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            creator: crate::creator::Creator::new(),
            to_write: std::collections::VecDeque::new(),
        }
    }

    /// Generates a new ttyrec frame with the given data.
    ///
    /// Equivalent to calling `frame_at` and passing
    /// `std::time::Instant::now()` as the `time` parameter.
    pub fn frame(&mut self, data: &[u8]) -> crate::error::Result<()> {
        self.frame_at(std::time::Instant::now(), data)
    }

    /// Generates a new ttyrec frame with the given data at the given time.
    ///
    /// The frame data will be stored on this object until written out by
    /// calls to `poll_write`.
    ///
    /// Note that this is not guaranteed to do the correct thing unless the
    /// `cur_time` parameters given in each `frame_at` call are
    /// non-decreasing.
    pub fn frame_at(
        &mut self,
        time: std::time::Instant,
        data: &[u8],
    ) -> crate::error::Result<()> {
        let frame = self.creator.frame_at(time, data);
        let bytes: Vec<u8> = std::convert::TryFrom::try_from(frame)?;
        self.to_write.extend(bytes.iter());
        Ok(())
    }

    /// Attempt to write serialized ttyrec frames to the underlying writer.
    ///
    /// Writes data from the previous calls to `frame` and `frame_at`. Returns
    /// `Ok(Async::Ready(()))` if all bytes were written, and
    /// `Ok(Async::NotReady)` otherwise.
    pub fn poll_write(&mut self) -> futures::Poll<(), crate::error::Error> {
        loop {
            if self.to_write.is_empty() {
                return Ok(futures::Async::Ready(()));
            }

            let (a, b) = self.to_write.as_slices();
            let buf = if a.is_empty() { b } else { a };

            let n = futures::try_ready!(self
                .writer
                .poll_write(buf)
                .context(crate::error::WriteFile));

            if n > 0 {
                for _ in 0..n {
                    self.to_write.pop_front();
                }
            } else {
                return Err(crate::error::Error::EOF);
            }
        }
    }

    /// Returns `true` if there are still bytes that need to be written.
    ///
    /// It is only necessary to call `poll_write` if this method returns
    /// `true`.
    pub fn needs_write(&self) -> bool {
        !self.to_write.is_empty()
    }
}
