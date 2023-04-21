use apalis::postgres::PostgresStorage;
use apalis::prelude::Storage;
use summarizer::error::Serror;
use summarizer::scheduler::Youtubelink;
use summarizer::utils::env_var;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Serror> {
    let postgres_url = env_var("DATABASE_URL")?;
    let you = Youtubelink("https://www.youtube.com/watch?v=GJLlxj_dtq8".to_string());
    let mut storage: PostgresStorage<Youtubelink> = PostgresStorage::connect(postgres_url).await?;
    let _job = storage
        .push(you)
        .await
        .map_err(|_| Serror::Database("asdf".to_string()))?;
    Ok(())
}
