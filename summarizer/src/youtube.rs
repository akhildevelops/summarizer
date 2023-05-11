use crate::error::Serror;
use regex::Regex;
use reqwest::Client;
use youtube_transcript::{Transcript, YoutubeBuilder};
pub struct Youtube<'a> {
    link: &'a str,
    video_id_extractor: Regex,
}
impl<'a> Youtube<'a> {
    pub fn link(link: &'a str) -> Result<Self, Serror> {
        Ok(Self {
            link,
            video_id_extractor: Regex::new(r"v=([^&]+)")?,
        })
    }

    fn video_id(&self) -> Result<&str, Serror> {
        let capture = self
            .video_id_extractor
            .captures(self.link)
            .ok_or(Serror::Youtubefetch(format!(
                "Cannot capture the video id in the url: {}",
                self.link
            )))?
            .get(1)
            .ok_or(Serror::Youtubefetch(format!(
                "Cannot capture the video id in the url: {}",
                self.link
            )))?;

        Ok(capture.as_str())
    }

    pub async fn content(&self) -> Result<YoutubeContent, Serror> {
        // Get Youtube Body
        let client = Client::default();
        let response = client.get(self.link).send().await?;
        let content = response.text().await?;

        Ok(YoutubeContent {
            content: content,
            video_id: self.video_id()?.to_string(),
        })
    }
}

#[derive(Debug)]
pub struct YoutubeContent {
    content: String,
    video_id: String,
}

impl YoutubeContent {
    /// Get transcript
    pub async fn transcirpt(&self) -> Result<Transcript, Serror> {
        YoutubeBuilder::default()
            .build()
            .transcript_from_text(&self.content)
            .await
            .map_err(|x| Serror::Youtubefetch(x.to_string()))
    }

    /// Get image link
    pub fn image_link(&self) -> String {
        format!("https://img.youtube.com/vi/{}/0.jpg", self.video_id)
    }

    /// Get title
    pub fn title(&self) -> Result<Option<String>, Serror> {
        let c = Regex::new(r"<title>(.+)</title>")?.captures(&self.content);
        if let Some(capture) = c {
            if let Some(match_str) = capture.get(1) {
                return Ok(Some(match_str.as_str().to_string()));
            }
        }
        Ok(None)
    }

    /// Get transcript in text
    pub async fn transcript_text(&self) -> Result<String, Serror> {
        let transcript = self.transcirpt().await?;
        Ok(transcript
            .transcripts
            .into_iter()
            .map(|x| format!("{} ", x.text))
            .collect::<String>())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    #[ignore = "Requires mocking youtube response"]
    async fn test_content() {
        let content = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
            .unwrap()
            .content()
            .await
            .unwrap()
            .transcript_text()
            .await
            .unwrap();
        println!("{}", content);
    }

    #[tokio::test]
    #[ignore = "Requires mocking youtube response"]
    async fn test_title() {
        let title = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
            .unwrap()
            .content()
            .await
            .unwrap()
            .title()
            .unwrap()
            .expect("No title found");
        println!("{}", title);
    }

    #[tokio::test]
    #[ignore = "Requires mocking youtube response"]
    async fn test_image() {
        let image_link = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8")
            .unwrap()
            .content()
            .await
            .unwrap()
            .image_link();
        println!("{}", image_link);
    }
}
