use summarizer::error::Serror;
use summarizer::scheduler::setup_youtube_data_workers;
use summarizer::utils::env_var;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Serror> {
    let postgres_url = env_var("DATABASE_URL")?;
    setup_youtube_data_workers(&postgres_url).await
}
