#![allow(non_snake_case)]

mod components;

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

use crate::components::{Challenge, HeroHeader, PageHeader};

#[derive(PartialEq, Props)]
pub struct CodeContentProps {
    pub content: String,
}

pub fn CodeContent(cx: Scope<CodeContentProps>) -> Element {
    cx.render(rsx!(pre {"{cx.props.content}"}))
}

async fn app_endpoint() -> Html<String> {
    fn app(cx: Scope) -> Element {
        let content = "fn main() {
    println!(&quot; Hello, world!&quot;); 
}"
        .to_string();

        cx.render(rsx!(div {
            Challenge{title: "Hi!".to_string(), assignment: "Write a program that outputs 'Hi!'.".to_string()}}))
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
