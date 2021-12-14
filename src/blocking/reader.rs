/// Reads ttyrec frames from a [`std::io::Read`] instance.
pub struct Reader<T: std::io::Read> {
    input: T,
    parser: crate::Parser,
    buf: [u8; 4096],
}

impl<T: std::io::Read> Reader<T> {
    /// Creates a new [`Reader`] from a [`std::io::Read`] instance.
    pub fn new(input: T) -> Self {
        Self {
            input,
            parser: crate::Parser::new(),
            buf: [0; 4096],
        }
    }

    /// Returns the next parsed frame from the input stream.
    ///
    /// # Errors
    /// * [`Error::EOF`](crate::Error::EOF): The input stream has been closed.
    /// * [`Error::Read`](crate::Error::Read): There was an error reading from
    /// the input stream.
    pub fn read_frame(&mut self) -> crate::Result<crate::Frame> {
        loop {
            if let Some(frame) = self.parser.next_frame() {
                return Ok(frame);
            }
            let bytes = self
                .input
                .read(&mut self.buf)
                .map_err(|source| crate::Error::Read { source })?;
            if bytes == 0 {
                return Err(crate::Error::EOF);
            }
            self.parser.add_bytes(
                // read() returning a value means that that many bytes are
                // guaranteed to be available
                self.buf.get(..bytes).unwrap_or_else(|| unreachable!()),
            );
        }
    }

    /// How much the timestamps in this file should be offset by.
    ///
    /// See [`Parser::offset`](crate::Parser::offset).
    pub fn offset(&self) -> Option<std::time::Duration> {
        self.parser.offset()
    }
}
