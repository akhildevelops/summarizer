use crate::database::Postgresmethods;
use crate::error::Serror;
use crate::youtube::Youtube;
use crate::Summarizer;
use apalis::layers::Extension;
use apalis::postgres::PostgresStorage;
use apalis::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::{fs, io::Write};
#[derive(Deserialize, Serialize)]
pub struct Youtubelink(pub String);

impl Job for Youtubelink {
    const NAME: &'static str = "youtube-transcript";
}

pub async fn transcript_summary(
    job: impl Into<Youtubelink>,
    ctx: JobContext,
) -> Result<(), Serror> {
    let pgpool = ctx
        .data_opt::<PgPool>()
        .ok_or_else(|| Serror::Other("Cannot observe Pgpool connection".to_string()))?;
    let summarizer = ctx
        .data_opt::<Summarizer>()
        .ok_or(Serror::Other("Summarizer cannot be found".to_string()))?;
    let pm = Postgresmethods::new(&pgpool);
    let youtube_link: Youtubelink = job.into();
    let youtube_content = Youtube::link(&youtube_link.0)?.content().await?;

    // description
    let description = youtube_content.transcript_text().await?;

    // title
    let title = youtube_content
        .title()?
        .unwrap_or("[Title Not Found]".to_string());

    // image
    let image = youtube_content.image_link();
    let response = reqwest::get(image).await?;
    let image_raw = response.bytes().await?;
    let path = "./data";
    std::fs::create_dir_all(path)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/{}.jpg", path, youtube_content.video_id))?;
    file.write_all(&image_raw)?;

    let remote_url = pm
        .insert_remoteurl(&youtube_link.0, &youtube_content.video_id, &title)
        .await?;

    let ts = pm.insert_transcript(&description, &remote_url).await?;
    let _summary = summarizer.summarize(&description).await?;
    pm.insert_transcriptsummary(&_summary, &ts).await?;
    Ok(())
}

pub async fn setup_youtube_data_workers(postgres_url: &str) -> Result<(), Serror> {
    let pgpool = PgPool::connect(&postgres_url).await?;
    let ps_client = PostgresStorage::<Youtubelink>::new(pgpool.clone());
    ps_client.setup().await?;
    let summarizer = Summarizer::default_params()?;
    ps_client.setup().await?;
    Monitor::new()
        .register_with_count(1, |_| {
            WorkerBuilder::new(ps_client.clone())
                .layer(Extension(pgpool.clone()))
                .layer(Extension(summarizer.clone()))
                .build_fn(transcript_summary)
        })
        .run()
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_write_file() {
        let path = "./data";
        std::fs::create_dir_all(path).unwrap();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}/{}.jpg", path, "asdf"))
            .unwrap();
        file.write_all(&vec![12, 23, 45]).unwrap();
    }
}
