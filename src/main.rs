#![allow(non_snake_case)]

mod server;
mod web;

use axum::Router;
use server::kits;

#[tokio::main]
async fn main() {
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

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting server on {}", address);

    axum::Server::bind(&address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
