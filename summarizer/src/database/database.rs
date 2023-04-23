use super::models::{Remoteurl, Transcript, TranscriptSummary};
use sqlx::postgres::PgPool;
use sqlx::Error;
pub(crate) struct Postgresmethods<'a> {
    client: &'a PgPool,
}

impl<'a> Postgresmethods<'a> {
    pub(crate) fn new(pool: &'a PgPool) -> Self {
        Self { client: pool }
    }
}

impl<'a> Postgresmethods<'a> {
    pub(crate) async fn insert_remoteurl(&self, url: &str) -> Result<Remoteurl, Error> {
        let insert_query = format!(
            "INSERT INTO remoteurl (link) VALUES ('{}') RETURNING *",
            url
        );
        sqlx::query_as::<_, Remoteurl>(&insert_query)
            .fetch_one(self.client)
            .await
    }
    pub(crate) async fn insert_transcript(
        &self,
        transcript: &str,
        remote_url: &Remoteurl,
    ) -> Result<Transcript, Error> {
        let insert_query = format!(
            "INSERT INTO transcript (remote_id,content) VALUES ({},'{}') RETURNING *",
            remote_url.id, transcript
        );
        sqlx::query_as::<_, Transcript>(&insert_query)
            .fetch_one(self.client)
            .await
    }
    pub(crate) async fn insert_transcriptsummary(
        &self,
        summary: &str,
        transcript: &Transcript,
    ) -> Result<TranscriptSummary, Error> {
        let insert_query = format!(
            "INSERT INTO transcriptsummary (transcript_id,content) VALUES ({},'{}') RETURNING *",
            transcript.id, summary
        );
        sqlx::query_as::<_, TranscriptSummary>(&insert_query)
            .fetch_one(self.client)
            .await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDateTime;
    #[tokio::test]
    async fn insert_trans() {
        let p_url = "postgres://postgres:postgres@db/summarizer";
        let pool = PgPool::connect(p_url).await.unwrap();
        let pmethods = Postgresmethods::new(&pool);
        let r_url = Remoteurl {
            id: 1,
            created_at: NaiveDateTime::MIN,
            link: "sadf".to_string(),
        };
        pmethods
            .insert_transcript("asdfdsaf", &r_url)
            .await
            .unwrap();
    }
}
