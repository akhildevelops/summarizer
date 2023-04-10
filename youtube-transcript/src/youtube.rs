use crate::config::Config;
use crate::parser::{HTMLParser, TranscriptParser};
use reqwest::Client;
use roxmltree::Document;
use std::error::Error;
struct Youtube<'a, 'b> {
    link: &'a str,
    config: &'b Config,
}

impl<'a, 'b> Youtube<'a, 'b> {
    fn link(link: &'a str, config: &'b Config) -> Self {
        Self { link, config }
    }
}

impl<'a, 'b> Youtube<'a, 'b> {
    async fn get_transcript(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::default();
        let response = client.get(self.link).send().await?;
        let c = response
            .text()
            .await?
            .caption(self.config.parser.from, self.config.parser.to)?;
        let response = client.get(c.base_url).send().await?;
        let trans_resp = response.text().await?;
        let doc = Document::parse(&trans_resp)?;
        let t = TranscriptParser::parse(&doc)?;
        Ok(t.describe())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Config;
    #[tokio::test]
    async fn test_u_trans() {
        let config = Config::default();
        let u = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8", &config);
        let transcript = u.get_transcript().await.unwrap();
        assert_eq!(transcript, "Hello");
    }
}
