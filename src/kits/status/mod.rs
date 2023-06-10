use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn status_endpoint() -> Json<Value> {
    Json(json!({"status": "READY"}))
}

pub struct Status {}

impl Status {
    pub fn new() -> Self {
        Self {}
    }

    pub fn router(&self) -> Router {
        Router::new().route("/", get(status_endpoint))
    }
}
