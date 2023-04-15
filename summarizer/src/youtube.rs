use std::error::Error;

use crate::summarize::Summarize;
use youtube_transcript::YoutubeBuilder;
pub struct Youtube<'a> {
    link: &'a str,
}
impl<'a> Youtube<'a> {
    pub fn link(link: &'a str) -> Self {
        Self { link }
    }
    pub async fn content(&self) -> Result<YoutubeContent, Box<dyn Error>> {
        let transcript = YoutubeBuilder::default()
            .build()
            .transcript(self.link)
            .await?;
        Ok(YoutubeContent {
            content: transcript.into(),
        })
    }
}
pub struct YoutubeContent {
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
    #[ignore = "Requires mocking youtube response"]
    async fn test_content() {
        let content = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
            .content()
            .await
            .unwrap();
        println!("{}", content.content);
    }
}
