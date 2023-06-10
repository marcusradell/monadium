#![allow(non_snake_case)]

use axum::Router;

mod kits;
mod web;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

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

    println!("Starting server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
