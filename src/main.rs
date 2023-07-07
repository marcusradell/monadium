#![allow(non_snake_case)]

mod server;
mod web;

use crate::server::io;
use axum::Router;
use server::io::config::Config;
use server::kits;
use tracing::info;

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() {
    io::tracing::init();

    let config = Config::init();

    let app_router = setup(config).await;

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Booting server on {}", address);

    let server = axum::Server::bind(&address).serve(app_router.into_make_service());

    info!("Server ready.");

    server.await.expect("Server failed.");
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn shuttle(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let config = Config::init(secrets);

    let app_router = setup(config).await;

    info!("Server ready.");

    Ok(app_router.into())
}

async fn setup(config: Config) -> Router {
    let _repo = io::repo::Repo::init(config.database_url, config.migrate_db).await;

    let courses_kit = kits::Courses::new();
    let modules_kit = kits::Modules::new();
    let challenges_kit = kits::Challenges::new();

    let api_router = Router::new()
        .nest("/challenges", challenges_kit.router())
        .nest("/courses", courses_kit.router())
        .nest("/modules", modules_kit.router());

    let status_kit = kits::Status::new();
    let web_kit = kits::Web::new();

    let app_router = Router::new()
        .nest("/status", status_kit.router())
        .nest("/api", api_router)
        .nest("/", web_kit.router());

    // TODO: it's the best point in time I know right now to set status to ready, but probably room for improvement.
    status_kit
        .set_value(kits::StatusValue::Ready)
        .expect("Failed to set server status.");

    app_router
}
