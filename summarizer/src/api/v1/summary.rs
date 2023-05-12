use crate::database::Postgresmethods;
use crate::error::Serror;
use crate::scheduler::Youtubelink;
use apalis::postgres::PostgresStorage;
use apalis::prelude::Storage;
use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json;
use sqlx::postgres::PgPool;
pub(crate) async fn summaries(State(pg): State<PgPool>) -> Result<Json<serde_json::Value>, Serror> {
    let pg = Postgresmethods::new(&pg);
    let summaries = pg.get_summaries().await?;
    Ok(Json(serde_json::to_value(summaries)?))
}

#[derive(Deserialize)]
pub struct Link {
    link: String,
}

pub(crate) async fn summarize(
    State(pg): State<PgPool>,
    Json(link): Json<Link>,
) -> Result<Json<serde_json::Value>, Serror> {
    let mut storage: PostgresStorage<Youtubelink> = PostgresStorage::new(pg);
    let you = Youtubelink(link.link.clone());
    let _job = storage
        .push(you)
        .await
        .map_err(|_| Serror::Database(format!("Cannot create a job for the link: {}", link.link)));
    let resp_json = format!("{{\"registerd_link\":\"{}\"}}", link.link);
    Ok(Json(serde_json::from_str(&resp_json)?))
}
