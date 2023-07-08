use axum::Router;

use crate::{
    io::{Config, Repo},
    kits::{KitRouter, Kits, StatusValue},
};

pub async fn setup(config: Config) -> Router {
    let repo = Repo::init(config.database_url, config.migrate_db).await;

    let kits = Kits::new(&repo);

    let api_router = Router::new().nest("/challenges", kits.challenges.router());

    let app_router = Router::new()
        .nest("/status", kits.status.router())
        .nest("/api", api_router)
        .nest("/", kits.web.router());

    // TODO: it's the best point in time I know right now to set status to ready, but probably room for improvement.
    kits.status
        .set_value(StatusValue::Ready)
        .expect("Failed to set server status.");

    app_router
}
