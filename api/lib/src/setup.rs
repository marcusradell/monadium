use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    io::{Config, Repo},
    kits::{KitRouter, Kits, StatusValue},
};

pub async fn setup(config: Config) -> Router {
    let repo = Repo::init(config.database_url, config.migrate_db).await;

    let kits = Kits::new(&repo);

    let api_router = Router::new().nest("/challenges", kits.challenges.router());

    let cors = CorsLayer::new().allow_origin(Any);

    let app_router = Router::new()
        .nest("/status", kits.status.router())
        .nest("/api", api_router)
        .layer(cors);

    // TODO: it's the best point in time I know right now to set status to ready, but probably room for improvement.
    kits.status
        .set_value(StatusValue::Ready)
        .expect("Failed to set server status.");

    app_router
}
