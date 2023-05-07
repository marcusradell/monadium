#![allow(non_snake_case)]

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

pub fn HeroHeader(cx: Scope) -> Element {
    cx.render(rsx!(h1 {"Monadium.org"}))
}

pub fn PageHeader(cx: Scope) -> Element {
    cx.render(rsx!(h2 {"Learning programming fundamentals with Rust"}))
}

pub fn ChapterHeader(cx: Scope) -> Element {
    cx.render(rsx!(h3 {"Challenge 1"}))
}

pub fn ParagraphHeader(cx: Scope) -> Element {
    cx.render(rsx!(p {"Run code with Rust"}))
}

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
            HeroHeader {}
            PageHeader {}
            ChapterHeader {}
            ParagraphHeader {}
            CodeContent{content: content}}))
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
