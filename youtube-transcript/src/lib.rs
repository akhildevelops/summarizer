#![warn(missing_docs)]
//! Asynchronous Youtube Transcript library
//! Examples:
//!
mod config;
mod error;
mod parser;
mod youtube;
pub use config::Config;
pub use youtube::Youtube;
