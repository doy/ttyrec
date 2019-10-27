//! This crate contains helpers for reading and writing
//! [ttyrec](https://en.wikipedia.org/wiki/Ttyrec) files.
//!
//! `Parser` and `Creator` can be used to read and write files manually, and
//! `Reader` and `Writer` are helpers to provide a nicer API for asynchronous
//! applications using `tokio`.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

mod creator;
pub use creator::Creator;
mod error;
pub use error::Error;
mod frame;
pub use frame::Frame;
mod parser;
pub use parser::Parser;
mod reader;
pub use reader::Reader;
mod writer;
pub use writer::Writer;
