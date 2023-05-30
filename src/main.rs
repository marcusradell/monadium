#![allow(non_snake_case)]

mod components;

use crate::components::{Challenge, PageHeader};
use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

async fn app_endpoint() -> Html<String> {
    fn app(cx: Scope) -> Element {
        cx.render(rsx!(
        div {
            PageHeader{
                text: "Learning programming fundamentals with Rust".to_string()
            }
            Challenge{
                title: "Hi!".to_string(),
                assignment: "Write a program that outputs 'Hi!'.".to_string()
            }
        }))
    }

    let mut app = VirtualDom::new(app);

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
