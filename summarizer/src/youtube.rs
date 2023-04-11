use std::error::Error;

use crate::Summarize;
use youtube_transcript::{Config, Youtube as Yt};
struct Youtube<'a> {
    link: &'a str,
}
impl<'a> Youtube<'a> {
    fn link(link: &'a str) -> Self {
        Self { link }
    }
    async fn content(&self) -> Result<YoutubeContent, Box<dyn Error>> {
        let c = Config::default();
        let transcript = Yt::link(self.link, &c).get_transcript().await?;
        Ok(YoutubeContent {
            content: transcript,
        })
    }
}
struct YoutubeContent {
    content: String,
}

impl Summarize for YoutubeContent {
    fn description(&self) -> &str {
        self.content.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_summarise_youtube_link() {
        Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
            .content()
            .await
            .unwrap()
            .summarize()
            .unwrap();
    }
}
