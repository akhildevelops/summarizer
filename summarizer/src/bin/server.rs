use axum;
use sqlx::postgres::PgPool;
use summarizer::api;
use summarizer::utils::env_var;
use tokio;
#[tokio::main]
async fn main() -> Result<(), String> {
    let postgres_url = env_var("DATABASE_URL").map_err(|x| x.to_string())?;
    let pg = PgPool::connect(&postgres_url)
        .await
        .map_err(|x| x.to_string())?;
    let router = api::get_router(pg);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .map_err(|x| x.to_string())?;
    Ok(())
}
