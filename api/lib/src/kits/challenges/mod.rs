use crate::io::Repo;
use crate::io::Result;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use sqlx::query_as;

use super::KitRouter;

#[derive(Serialize)]
pub struct Challenge {
    id: u64,
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
        let rows = query_as!(Challenge, "select id, name, content from challenges")
            .fetch_all(self.repo.pool.as_ref())
            .await?;

        Ok(rows)
    }

    pub async fn add(&self, name: String, content: String) -> Result<()> {
        sqlx::query!(
            "insert into challenges (name, content) values ( ?, ?)",
            name,
            content
        )
        .execute(self.repo.pool.as_ref())
        .await?;

        Ok(())
    }
}

impl KitRouter for ChallengesKit {
    fn router(&self) -> Router {
        Router::new()
            .route(
                "/get_all",
                get({
                    let this = self.clone();

                    || async move { Json(this.get_all().await.unwrap()) }
                }),
            )
            .route(
                "/add",
                post({
                    let this = self.clone();

                    || async move {
                        this.add("Name".to_string(), "Content...".to_string())
                            .await
                            .unwrap()
                    }
                }),
            )
    }
}
