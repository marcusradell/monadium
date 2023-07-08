use std::sync::Arc;

use super::result::{Error, Result};
use sqlx::{mysql::MySqlPoolOptions, sqlx_macros::migrate, MySqlPool};

#[derive(Clone)]
pub struct Repo {
    pub pool: Arc<MySqlPool>,
}

impl Repo {
    #[tracing::instrument]
    pub async fn init(db_uri: String, migrate_db: bool) -> Self {
        tracing::info!("DB initializing...");

        let pool = MySqlPoolOptions::new().connect(&db_uri).await.unwrap();

        let _row: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await
            .expect("Failed while ensuring DB connection.");

        tracing::info!("DB connection ensured.");

        if migrate_db {
            migrate(&pool).await.unwrap();
        } else {
            tracing::info!("Migrations skipped.");
        }

        tracing::info!("DB initialized!");

        Self {
            pool: Arc::new(pool),
        }
    }
}

pub async fn migrate(pool: &MySqlPool) -> Result<()> {
    tracing::info!("Migrations started...");

    migrate!("db/migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations.");

    tracing::info!("Migrated DB!");

    Ok(())
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("{:?}", error);
        Error::internal_error()
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        tracing::error!("{:?}", error);
        Error::internal_error()
    }
}
