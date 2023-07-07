#![allow(non_snake_case)]

mod server;
mod web;

use crate::server::io::{self};
use axum::Router;
use server::kits;
use tracing::info;
// use tracing::{info, Level};
// use tracing_subscriber::FmtSubscriber;

// #[tokio::main]
// async fn main() {
#[shuttle_runtime::main]
async fn shuttle(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // dotenv().ok();

    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::INFO)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("Failed to set global default tracing subscriber.");

    let db_url = secrets
        .get("DATABASE_URL")
        .expect("Missing secret DATABASE_URL");

    let migrate_db = secrets.get("MIGRATE_DB").expect("Missing MIGRATE_DB") == "ON";

    let _pool = io::db::init(db_url, migrate_db).await;

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

    // let address = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    // info!("Booting server on {}", address);

    // let server = axum::Server::bind(&address).serve(app_router.into_make_service());

    status_kit
        .set_value(kits::StatusValue::Ready)
        .expect("Failed to set server status.");

    info!("Server ready.");

    // server.await.expect("Server failed.");

    Ok(app_router.into())
}
