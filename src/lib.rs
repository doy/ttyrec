//! This crate contains helpers for reading and writing
//! [ttyrec](https://en.wikipedia.org/wiki/Ttyrec) files.
//!
//! `Parser` and `Creator` can be used to read and write files manually, and
//! `Reader` and `Writer` are helpers to provide a nicer API for asynchronous
//! applications using futures. Additionally, `blocking::Reader` and
//! `blocking::Writer` provide a similar API for non-asynchronous
//! applications.
//!
//! If you do not need the async API, the `futures` dependency can be removed
//! by building with `default_features = false` (by default, the `"async"`
//! feature is enabled).

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::as_conversions)]
#![warn(clippy::get_unwrap)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::similar_names)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::type_complexity)]

mod creator;
pub use creator::Creator;
mod error;
pub use error::{Error, Result};
mod frame;
pub use frame::Frame;
mod parser;
pub use parser::Parser;
pub mod blocking;
#[cfg(feature = "async")]
mod reader;
#[cfg(feature = "async")]
pub use reader::Reader;
#[cfg(feature = "async")]
mod writer;
#[cfg(feature = "async")]
pub use writer::Writer;
