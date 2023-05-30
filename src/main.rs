#![allow(non_snake_case)]

mod components;

use axum::{response::Html, routing::get, Router};
use components::App;
use dioxus::prelude::*;

async fn app_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(App);

    let _ = app.rebuild();

    Html(dioxus_ssr::render(&app))
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}
