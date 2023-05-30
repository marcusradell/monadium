#![allow(non_snake_case)]

mod components;

use axum::{response::Html, routing::get, Json, Router};
use components::App;
use dioxus::prelude::*;
use serde_json::{json, Value};

async fn status_endpoint() -> Json<Value> {
    Json(json!({"status": "READY"}))
}

async fn api_endpoint() -> Json<Value> {
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

    let api_router = Router::new().route("/", get(api_endpoint));

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
