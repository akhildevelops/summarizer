use axum;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use summarizer::api;
use summarizer::utils::env_var;
use tokio;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
#[tokio::main]
async fn main() -> Result<(), String> {
    // setup tracing for debugging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // connect to postgres
    let postgres_url = env_var("DATABASE_URL").map_err(|x| x.to_string())?;
    let pg = PgPool::connect(&postgres_url)
        .await
        .map_err(|x| x.to_string())?;

    // Setup server
    let app = api::get_router(pg);
    let cors = CorsLayer::permissive();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app.layer(TraceLayer::new_for_http())
                .layer(cors)
                .into_make_service(),
        )
        .await
        .unwrap();
    Ok(())
}
