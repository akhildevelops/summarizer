#![warn(missing_docs)]
//! # Asynchronous Youtube Transcript library
//! Get transcripts / captions of videos.
//! ### Downloading transcript from Youtube:
//! ```rust
//! let link:&str = "https://www.youtube.com/watch?v=RcYjXbSJBN8";
//!
//! # Create a youtube instance from builder.
//! let youtube_loader:Youtube = YoutubeBuilder::default().build();
//!
//! # Get the transcript by loading youtube url.
//! let transcript:Transcript = youtube_loader.transcript(link).await?;
//! ```
//!
mod config;
mod utils;

mod error;
mod parser;
mod youtube;
pub use config::Config;
pub use parser::{Transcript, TranscriptCore};
pub use youtube::{Youtube, YoutubeBuilder};
