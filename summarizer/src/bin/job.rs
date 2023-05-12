use apalis::postgres::PostgresStorage;
use apalis::prelude::Storage;
use summarizer::error::Serror;
use summarizer::scheduler::Youtubelink;
use summarizer::utils::env_var;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Serror> {
    let postgres_url = env_var("DATABASE_URL")?;
    let links = [
        // "https://www.youtube.com/watch?v=sBH-ngpL0zo",
        "https://www.youtube.com/watch?v=WYNRt-AwoUg",
        // "https://www.youtube.com/watch?v=fPWzeFYtjfc",
        // "https://www.youtube.com/watch?v=e64JTo7LMmY",
        // "https://www.youtube.com/watch?v=vcTtYZX7Ahk",
        // "https://www.youtube.com/watch?v=FzUeQ7AZ3O0",
        // "https://www.youtube.com/watch?v=sWSgQFmWMxw",
        // "https://www.youtube.com/watch?v=vjfypJq5pK8",
    ];
    let mut storage: PostgresStorage<Youtubelink> = PostgresStorage::connect(postgres_url).await?;
    for link in links {
        let you = Youtubelink(link.to_string());
        let _job = storage
            .push(you)
            .await
            .map_err(|_| Serror::Database("asdf".to_string()))?;
    }
    Ok(())
}
