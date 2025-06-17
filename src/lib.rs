//! # bhlang-rs
//!
//! A Rust library for reading and writing Brawlhalla's lang (.bin) files.
//!
//! # Organization
//!
//! This library exports the following:
//! * `LangFile`: A struct that represents the data in a lang file.

mod lang_file;

// Re-exports
pub use lang_file::*;
