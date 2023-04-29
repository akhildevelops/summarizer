use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub(crate) struct Remoteurl {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub link: String,
}

#[derive(FromRow)]
pub(crate) struct Transcript {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub remote_id: i32,
}

#[derive(FromRow)]
pub(crate) struct TranscriptSummary {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub transcript_id: i32,
}
