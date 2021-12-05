use futures_lite::io::AsyncWriteExt as _;

/// Writes ttyrec frames to a [`futures_lite::io::AsyncWrite`] instance.
pub struct Writer<T: futures_lite::io::AsyncWrite> {
    output: T,
    creator: crate::Creator,
}

impl<T: futures_lite::io::AsyncWrite + std::marker::Unpin + Send> Writer<T> {
    /// Creates a new [`Writer`](Self) from a [`futures_lite::io::AsyncWrite`]
    /// instance.
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
    pub async fn frame(&mut self, data: &[u8]) -> crate::Result<()> {
        self.frame_at(std::time::Instant::now(), data).await
    }

    /// Writes a new frame to the output stream, using the given time and
    /// data.
    ///
    /// # Errors
    /// * [`Error::Write`](crate::Error::Write): There was an error writing to
    /// the input stream.
    pub async fn frame_at(
        &mut self,
        cur_time: std::time::Instant,
        data: &[u8],
    ) -> crate::Result<()> {
        let frame = self.creator.frame_at(cur_time, data);
        let bytes: Vec<u8> = frame.try_into()?;
        self.output
            .write_all(&bytes)
            .await
            .map_err(|source| crate::Error::Write { source })
    }
}
