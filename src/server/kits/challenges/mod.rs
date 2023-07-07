use crate::io::result::Result;
use crate::server::io::repo::Repo;
use axum::{routing::get, Json, Router};
use serde::Serialize;
use sqlx::{query_as, types::Uuid};

#[derive(Serialize)]
pub struct Challenge {
    id: Uuid,
    name: String,
    content: String,
}

#[derive(Clone)]
pub struct ChallengesKit {
    repo: Repo,
}

impl ChallengesKit {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Challenge>> {
        let rows = query_as!(
            Challenge,
            "select id, name, content from challenges.challenges"
        )
        .fetch_all(self.repo.pool.as_ref())
        .await?;

        Ok(rows)
    }

    pub fn router(&self) -> Router {
        Router::new().route(
            "/get_all",
            get({
                let this = self.clone();

                || async move { Json(this.get_all().await.unwrap()) }
            }),
        )
    }
}
