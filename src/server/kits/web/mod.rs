use axum::{response::Html, routing::get, Router};
use dioxus::prelude::VirtualDom;

use crate::web::App;

use super::KitRouter;

async fn web_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(App);

    let _ = app.rebuild();

    Html(dioxus_ssr::render(&app))
}

pub struct WebKit {}

impl WebKit {
    pub fn new() -> Self {
        Self {}
    }
}

impl KitRouter for WebKit {
    fn router(&self) -> Router {
        Router::new()
            .route("/", get(web_endpoint))
            .route("/*path", get(web_endpoint))
    }
}
