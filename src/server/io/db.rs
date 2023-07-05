use std::path::Path;

use super::{
    env::ensure_env,
    result::{Error, Result},
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[tracing::instrument]
pub async fn init() -> Pool<Postgres> {
    tracing::info!("DB initializing...");

    let db_uri = ensure_env("DATABASE_URL");
    let migrate_db = ensure_env("MIGRATE_DB") == "ON";

    let pool = PgPoolOptions::new().connect(&db_uri).await.unwrap();

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

    pool
}

pub async fn migrate(pool: &Pool<Postgres>) -> Result<()> {
    tracing::info!("Migrations started...");

    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .expect("Migrator could not be created.")
        .run(pool)
        .await?;

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
