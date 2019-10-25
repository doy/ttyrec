#[repr(packed)]
struct Header {
    secs: u32,
    micros: u32,
    len: u32,
}

impl Header {
    fn time(&self) -> std::time::Duration {
        std::time::Duration::from_micros(u64::from(
            self.secs * 1_000_000 + self.micros,
        ))
    }

    fn len(&self) -> usize {
        self.len as usize
    }
}

pub struct Parser {
    reading: std::collections::VecDeque<u8>,
    read_state: Option<Header>,
}

impl Parser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.reading.extend(bytes.iter());
    }

    pub fn next_frame(&mut self) -> Option<crate::frame::Frame> {
        let header = if let Some(header) = &self.read_state {
            header
        } else {
            if self.reading.len() < std::mem::size_of::<Header>() {
                return None;
            }

            let secs1 = self.reading.pop_front().unwrap();
            let secs2 = self.reading.pop_front().unwrap();
            let secs3 = self.reading.pop_front().unwrap();
            let secs4 = self.reading.pop_front().unwrap();
            let secs = u32::from_le_bytes([secs1, secs2, secs3, secs4]);

            let usecs1 = self.reading.pop_front().unwrap();
            let usecs2 = self.reading.pop_front().unwrap();
            let usecs3 = self.reading.pop_front().unwrap();
            let usecs4 = self.reading.pop_front().unwrap();
            let micros = u32::from_le_bytes([usecs1, usecs2, usecs3, usecs4]);

            let len1 = self.reading.pop_front().unwrap();
            let len2 = self.reading.pop_front().unwrap();
            let len3 = self.reading.pop_front().unwrap();
            let len4 = self.reading.pop_front().unwrap();
            let len = u32::from_le_bytes([len1, len2, len3, len4]);

            let header = Header { secs, micros, len };
            self.read_state = Some(header);
            self.read_state.as_ref().unwrap()
        };

        if self.reading.len() < header.len() {
            return None;
        }

        let mut data = vec![];
        for _ in 0..header.len() {
            data.push(self.reading.pop_front().unwrap());
        }

        let time = header.time();

        self.read_state = None;
        Some(crate::frame::Frame { time, data })
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            reading: std::collections::VecDeque::new(),
            read_state: None,
        }
    }
}
