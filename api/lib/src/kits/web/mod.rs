use axum::{routing::get, Router};

use super::KitRouter;

async fn web_endpoint() -> &'static str {
    "<H1>TODO</H1>"
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
