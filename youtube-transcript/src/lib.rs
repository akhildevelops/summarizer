// #![warn(missing_docs)]
//! # Asynchronous Youtube Transcript library
//! Get transcripts / captions of videos.
//! ### Downloading transcript from Youtube:
//! ```rust
//! #let config = Config::default();
//! #let transcript = Youtube::link("https://www.youtube.com/watch?v=L_Guz73e6fw")
//! #                 .get_transcript().await?
//! #println!("transcript: {:?}", transcript);
//! ```
//!
mod config;
mod utils;

mod error;
mod parser;
mod youtube;
pub use config::Config;
pub use youtube::{Youtube, YoutubeBuilder};
