use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

async fn app_endpoint() -> Html<String> {
    fn app(cx: Scope) -> Element {
        let code_content = "fn main() {
    println!(&quot; Hello, world!&quot;); 
}";
        cx.render(rsx!(div {
            h1 {"Monadium.org"}
            h2 {"Learning programming fundamentals with Rust"}
            h3 {"Challenge 1"}
            p {"Run code with Rust"}
            pre {code_content}}))
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
