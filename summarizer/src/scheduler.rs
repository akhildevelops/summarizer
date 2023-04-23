use crate::error::Serror;
use crate::youtube::Youtube;
use crate::Summarizer;
use crate::{database::Postgresmethods, summarize::Summarize};
use apalis::layers::Extension;
use apalis::postgres::PostgresStorage;
use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
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
    let pm = Postgresmethods::new(&pgpool);
    let youtube_link: Youtubelink = job.into();
    let remote_url = pm.insert_remoteurl(&youtube_link.0).await?;
    let content = Youtube::link(&youtube_link.0).content().await?;
    let ts = pm
        .insert_transcript(content.description(), &remote_url)
        .await?;
    let _summary = Summarizer::summarize(&content).await?;
    let content = _summary
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| Serror::OpenAIError("Cannot find summary".to_string()))?
        .message
        .content;
    pm.insert_transcriptsummary(&content, &ts).await?;
    Ok(())
}

pub async fn setup_youtube_data_workers(postgres_url: &str) -> Result<(), Serror> {
    let pgpool = PgPool::connect(&postgres_url).await?;
    let ps_client = PostgresStorage::<Youtubelink>::new(pgpool.clone());
    ps_client.setup().await?;
    Monitor::new()
        .register_with_count(1, |_| {
            WorkerBuilder::new(ps_client.clone())
                .layer(Extension(pgpool.clone()))
                .build_fn(transcript_summary)
        })
        .run()
        .await?;
    Ok(())
}
