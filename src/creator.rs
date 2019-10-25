use std::convert::TryFrom as _;

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
    ) -> crate::error::Result<Vec<u8>> {
        let base_time = if let Some(base_time) = &self.base_time {
            base_time
        } else {
            self.base_time = Some(cur_time);
            self.base_time.as_ref().unwrap()
        };
        Vec::<u8>::try_from(crate::frame::Frame {
            time: cur_time - *base_time,
            data: data.to_vec(),
        })
    }
}

impl Default for Creator {
    fn default() -> Self {
        Self { base_time: None }
    }
}
