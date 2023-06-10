use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn placeholder_endpoint() -> Json<Value> {
    Json(json!({"ok": false, "code": "NOT_IMPLEMENTED"}))
}

pub struct Courses {}

impl Courses {
    pub fn new() -> Self {
        Self {}
    }

    pub fn router(&self) -> Router {
        Router::new().route("/get_all", get(placeholder_endpoint))
    }
}
