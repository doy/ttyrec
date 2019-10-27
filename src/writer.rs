use snafu::ResultExt as _;

pub struct Writer<W> {
    writer: W,
    creator: crate::creator::Creator,
    to_write: std::collections::VecDeque<u8>,
}

impl<W: tokio::io::AsyncWrite> Writer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            creator: crate::creator::Creator::new(),
            to_write: std::collections::VecDeque::new(),
        }
    }

    pub fn frame(&mut self, data: &[u8]) -> crate::error::Result<()> {
        self.frame_at(std::time::Instant::now(), data)
    }

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

    pub fn poll_write(&mut self) -> futures::Poll<(), crate::error::Error> {
        let (a, b) = self.to_write.as_slices();
        let buf = if a.is_empty() { b } else { a };
        let n = futures::try_ready!(self
            .writer
            .poll_write(buf)
            .context(crate::error::WriteFile));
        for _ in 0..n {
            self.to_write.pop_front();
        }
        Ok(futures::Async::Ready(()))
    }

    pub fn is_empty(&self) -> bool {
        self.to_write.is_empty()
    }
}
