pub struct Frame {
    pub time: std::time::Duration,
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<Frame> for Vec<u8> {
    type Error = crate::error::Error;

    fn try_from(frame: Frame) -> crate::error::Result<Self> {
        let secs = u32::try_from(frame.time.as_secs()).map_err(|_| {
            crate::error::Error::FrameTooLong {
                input: frame.time.as_secs(),
            }
        })?;
        let micros = frame.time.subsec_micros();
        let len = u32::try_from(frame.data.len()).map_err(|_| {
            crate::error::Error::FrameTooBig {
                input: frame.data.len(),
            }
        })?;
        let mut bytes = vec![];
        bytes.extend(secs.to_le_bytes().iter());
        bytes.extend(micros.to_le_bytes().iter());
        bytes.extend(len.to_le_bytes().iter());
        bytes.extend(frame.data.iter());
        Ok(bytes)
    }
}
