#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom as _;

    #[test]
    fn test_basic() {
        let tests = vec![
            (
                Frame {
                    time: std::time::Duration::new(0, 0),
                    data: vec![],
                },
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                Frame {
                    time: std::time::Duration::new(38, 123_456_000),
                    data: b"\x1b[2Jfoobar".to_vec(),
                },
                vec![
                    38, 0, 0, 0, 64, 226, 1, 0, 10, 0, 0, 0, 27, 91, 50, 74,
                    102, 111, 111, 98, 97, 114,
                ],
            ),
        ];
        for (frame, bytes) in tests {
            assert_eq!(Vec::<u8>::try_from(frame).unwrap(), bytes);
        }
    }
}
