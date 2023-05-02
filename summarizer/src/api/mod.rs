mod v1;
use axum::{routing::get, Router};
use sqlx::postgres::PgPool;
use v1::summary::summaries;
pub fn get_router(pgpool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Summarizer" }))
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/summaries", get(summaries))
                    .with_state(pgpool),
            ),
        )
}
