//! This crate contains helpers for reading and writing
//! [ttyrec](https://en.wikipedia.org/wiki/Ttyrec) files.
//!
//! `Parser` and `Creator` can be used to read and write files manually, and
//! `Reader` and `Writer` are helpers to provide a nicer API for asynchronous
//! applications using `tokio`.

// XXX this is broken with ale
// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::multiple_crate_versions)]

mod creator;
pub use creator::Creator;
mod error;
pub use error::Error;
mod frame;
pub use frame::Frame;
mod parser;
pub use parser::Parser;
#[cfg(feature = "async")]
mod reader;
#[cfg(feature = "async")]
pub use reader::Reader;
#[cfg(feature = "async")]
mod writer;
#[cfg(feature = "async")]
pub use writer::Writer;
