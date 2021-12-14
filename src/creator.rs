/// Creates ttyrec frames.
///
/// A ttyrec file is a stream of concatenated frames. This struct creates
/// [`Frame`](crate::Frame) objects, which can be serialized to bytes using
/// their `try_from` implementation, and then a ttyrec file can be generated
/// by concatenating those byte strings.
#[derive(Debug, Clone, Default)]
pub struct Creator {
    base_time: Option<std::time::Instant>,
}

impl Creator {
    /// Creates a new [`Creator`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new [`Frame`](crate::Frame) object containing the given
    /// data.
    ///
    /// Equivalent to calling [`frame_at`](Self::frame_at) and passing
    /// [`std::time::Instant::now()`] as the `cur_time` parameter.
    pub fn frame(&mut self, data: &[u8]) -> crate::frame::Frame {
        self.frame_at(std::time::Instant::now(), data)
    }

    /// Returns a new [`Frame`](crate::Frame) object containing the given data
    /// at the given time.
    ///
    /// Note that this is not guaranteed to do the correct thing unless the
    /// `cur_time` parameters given in each [`frame_at`](Self::frame_at) call
    /// are non-decreasing.
    pub fn frame_at(
        &mut self,
        cur_time: std::time::Instant,
        data: &[u8],
    ) -> crate::frame::Frame {
        let base_time = if let Some(base_time) = &self.base_time {
            base_time
        } else {
            self.base_time = Some(cur_time);
            &cur_time
        };
        crate::frame::Frame {
            time: cur_time - *base_time,
            data: data.to_vec(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut creator = Creator::new();
        let base_time = std::time::Instant::now();

        let zero_frame: Vec<u8> =
            TryFrom::try_from(creator.frame_at(base_time, b"")).unwrap();
        assert_eq!(zero_frame, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data_frame: Vec<u8> = TryFrom::try_from(creator.frame_at(
            base_time + std::time::Duration::new(38, 123_456_000),
            b"\x1b[2Jfoobar",
        ))
        .unwrap();
        assert_eq!(
            data_frame,
            vec![
                38, 0, 0, 0, 64, 226, 1, 0, 10, 0, 0, 0, 27, 91, 50, 74, 102,
                111, 111, 98, 97, 114,
            ],
        );
    }
}
