#[derive(Debug, Clone)]
pub struct Creator {
    base_time: Option<std::time::Instant>,
}

impl Creator {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn frame(
        &mut self,
        cur_time: std::time::Instant,
        data: &[u8],
    ) -> crate::frame::Frame {
        let base_time = if let Some(base_time) = &self.base_time {
            base_time
        } else {
            self.base_time = Some(cur_time);
            self.base_time.as_ref().unwrap()
        };
        crate::frame::Frame {
            time: cur_time - *base_time,
            data: data.to_vec(),
        }
    }
}

impl Default for Creator {
    fn default() -> Self {
        Self { base_time: None }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut creator = Creator::new();
        let base_time = std::time::Instant::now();

        let zero_frame: Vec<u8> =
            std::convert::TryFrom::try_from(creator.frame(base_time, b""))
                .unwrap();
        assert_eq!(zero_frame, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let data_frame: Vec<u8> =
            std::convert::TryFrom::try_from(creator.frame(
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
