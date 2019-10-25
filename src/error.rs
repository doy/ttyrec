#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("failed to create ttyrec frame: got {} bytes of data, but ttyrec frames can be at most {} bytes", input, u32::max_value()))]
    FrameTooBig { input: usize },

    #[snafu(display("failed to create ttyrec frame: got {} seconds, but ttyrecs can be at most {} seconds", input, u32::max_value()))]
    FrameTooLong { input: u64 },

    #[snafu(display("failed to read from file: {}", source))]
    ReadFile { source: tokio::io::Error },

    #[snafu(display("failed to write to file: {}", source))]
    WriteFile { source: tokio::io::Error },
}

pub type Result<T> = std::result::Result<T, Error>;
