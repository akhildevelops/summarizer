mod v1;
use axum::{routing::get, Router};
use sqlx::postgres::PgPool;
use tower_http::services::{ServeDir, ServeFile};
use v1::summaries;
pub fn get_router(pgpool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Summarizer" }))
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/summaries", get(summaries))
                    .nest_service(
                        "/thumbnails",
                        ServeDir::new("/workspace/summarizer/data").not_found_service(
                            ServeFile::new("/workspace/summarizer/data/404.html"),
                        ),
                    )
                    .with_state(pgpool),
            ),
        )
}
