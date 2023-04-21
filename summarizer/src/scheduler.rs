use crate::error::Serror;
use crate::youtube::{Youtube, YoutubeContent};
use apalis::postgres::PostgresStorage;
use apalis::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Youtubelink(pub String);

impl Job for Youtubelink {
    const NAME: &'static str = "youtube-transcript";
}

pub async fn transcript(
    job: impl Into<Youtubelink>,
    _: JobContext,
) -> Result<YoutubeContent, Serror> {
    Youtube::link(&job.into().0).content().await
}

pub async fn setup_youtube_data_workers(postgres_url: &str) -> Result<(), Serror> {
    let ps_client = PostgresStorage::<Youtubelink>::connect(postgres_url).await?;
    ps_client.setup().await?;
    Monitor::new()
        .register_with_count(2, |_| {
            WorkerBuilder::new(ps_client.clone()).build_fn(transcript)
        })
        .run()
        .await?;
    Ok(())
}
