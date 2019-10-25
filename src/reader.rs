use snafu::ResultExt as _;

pub struct Reader<R> {
    reader: R,
    parser: crate::parser::Parser,
    read_buf: [u8; 4096],
    done_reading: bool,
}

impl<R: tokio::io::AsyncRead> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            parser: crate::parser::Parser::new(),
            read_buf: [0; 4096],
            done_reading: false,
        }
    }

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
}
