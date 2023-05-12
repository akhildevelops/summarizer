mod v1;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPool;
use tower_http::services::{ServeDir, ServeFile};
use v1::{summaries, summarize};
pub fn get_router(pgpool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Summarizer" }))
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/summarize", post(summarize))
                    .route("/summaries", get(summaries))
                    .nest_service(
                        "/thumbnails",
                        ServeDir::new("/workspace/summarizer/data")
                            .fallback(ServeFile::new("/workspace/summarizer/data/notfound.jpg")),
                    )
                    .with_state(pgpool),
            ),
        )
}
