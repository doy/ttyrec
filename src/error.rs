/// Errors potentially returned by this crate.
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    /// eof
    #[snafu(display("eof"))]
    EOF,

    /// failed to create ttyrec frame: got N bytes of data but ttyrec frames
    /// can be at most M bytes
    #[snafu(display("failed to create ttyrec frame: got {} bytes of data, but ttyrec frames can be at most {} bytes", input, u32::max_value()))]
    FrameTooBig { input: usize },

    /// failed to create ttyrec frame: got N seconds but ttyrec frames can be
    /// at most M seconds
    #[snafu(display("failed to create ttyrec frame: got {} seconds, but ttyrecs can be at most {} seconds", input, u32::max_value()))]
    FrameTooLong { input: u64 },

    /// failed to read from file
    #[cfg(feature = "async")]
    #[snafu(display("failed to read from file: {}", source))]
    ReadFile { source: tokio::io::Error },

    /// failed to write to file
    #[cfg(feature = "async")]
    #[snafu(display("failed to write to file: {}", source))]
    WriteFile { source: tokio::io::Error },
}

pub type Result<T> = std::result::Result<T, Error>;
