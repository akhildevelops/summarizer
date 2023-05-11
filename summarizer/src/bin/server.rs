use axum;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use summarizer::api;
use summarizer::utils::env_var;
use tokio;
use tower_http::trace::TraceLayer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let postgres_url = env_var("DATABASE_URL").map_err(|x| x.to_string())?;
    let pg = PgPool::connect(&postgres_url)
        .await
        .map_err(|x| x.to_string())?;
    let app = api::get_router(pg);
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(router.into_make_service())
    //     .await
    //     .map_err(|x| x.to_string())?;
    // Ok(())
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
    Ok(())
}
