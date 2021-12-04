/// Writes ttyrec frames to a [`std::io::Write`] instance.
pub struct Writer<T: std::io::Write> {
    output: T,
    creator: crate::Creator,
}

impl<T: std::io::Write> Writer<T> {
    /// Creates a new [`Writer`] from a [`std::io::Write`] instance.
    pub fn new(output: T) -> Self {
        Self {
            output,
            creator: crate::Creator::new(),
        }
    }

    /// Writes a new frame to the output stream, using the current time and
    /// given data.
    ///
    /// # Errors
    /// * [`Error::Write`](crate::Error::Write): There was an error writing to
    /// the input stream.
    pub fn frame(&mut self, data: &[u8]) -> crate::Result<()> {
        self.frame_at(std::time::Instant::now(), data)
    }

    /// Writes a new frame to the output stream, using the given time and
    /// data.
    ///
    /// # Errors
    /// * [`crate::Error::Write`](crate::Error::Write): There was an error
    /// writing to the input stream.
    pub fn frame_at(
        &mut self,
        cur_time: std::time::Instant,
        data: &[u8],
    ) -> crate::Result<()> {
        let frame = self.creator.frame_at(cur_time, data);
        let bytes: Vec<u8> = frame.try_into()?;
        self.output
            .write_all(&bytes)
            .map_err(|source| crate::Error::Write { source })
    }
}
