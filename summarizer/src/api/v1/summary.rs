use crate::database::Postgresmethods;
use crate::error::Serror;
use axum::extract::{Json, State};
use serde_json;
use sqlx::postgres::PgPool;

pub(crate) async fn summaries(State(pg): State<PgPool>) -> Result<Json<serde_json::Value>, Serror> {
    let pg = Postgresmethods::new(&pg);
    let summaries = pg.get_summaries().await?;
    Ok(Json(serde_json::to_value(summaries)?))
}
