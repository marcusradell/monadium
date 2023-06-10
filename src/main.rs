#![allow(non_snake_case)]

mod components;
mod services;

use axum::{response::Html, routing::get, Json, Router};
use components::App;
use dioxus::prelude::*;
use serde_json::{json, Value};

async fn status_endpoint() -> Json<Value> {
    Json(json!({"status": "READY"}))
}

async fn placeholder_endpoint() -> Json<Value> {
    Json(json!({"ok": false, "code": "NOT_IMPLEMENTED"}))
}

async fn web_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(App);

    let _ = app.rebuild();

    Html(dioxus_ssr::render(&app))
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);

    let status_router = Router::new().route("/", get(status_endpoint));

    let challenges_router = Router::new().route("/get_all", get(placeholder_endpoint));

    let courses_service = services::Courses::new();

    let modules_service = services::Modules::new();

    let api_router = Router::new()
        .nest("/challenges", challenges_router)
        .nest("/courses", courses_service.router())
        .nest("/modules", modules_service.router());

    let web_router = Router::new()
        .route("/", get(web_endpoint))
        .route("/*path", get(web_endpoint));

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .nest("/status", status_router)
                .nest("/api", api_router)
                .nest("/", web_router)
                .into_make_service(),
        )
        .await
        .unwrap();
}
