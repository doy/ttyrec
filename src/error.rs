/// Errors potentially returned by this crate.
#[derive(Debug)]
pub enum Error {
    /// failed to create ttyrec frame: got N bytes of data but ttyrec frames
    /// can be at most M bytes
    FrameTooBig { input: usize },

    /// failed to create ttyrec frame: got N seconds but ttyrec frames can be
    /// at most M seconds
    FrameTooLong { input: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrameTooBig { input } => write!(f, "failed to create ttyrec frame: got {} bytes of data, but ttyrec frames can be at most {} bytes", input, u32::max_value()),
            Self::FrameTooLong { input } => write!(f, "failed to create ttyrec frame: got {} seconds, but ttyrecs can be at most {} seconds", input, u32::max_value()),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
