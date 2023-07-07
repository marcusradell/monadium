use std::sync::{Arc, Mutex};

use crate::{io::result::Result, server::io::result::Error};
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum StatusValue {
    Booting,
    Ready,
    Unhealthy,
    _ShuttingDown,
}

#[derive(Clone)]
pub struct Status {
    value: Arc<Mutex<StatusValue>>,
}

impl Status {
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(StatusValue::Booting)),
        }
    }

    pub async fn get_value(&self) -> StatusValue {
        let value = self.value.lock().map(|value| (*value).clone());
        value.unwrap_or(StatusValue::Unhealthy)
    }

    pub fn set_value(&self, status_value: StatusValue) -> Result<()> {
        let mut value = self.value.lock().map_err(|_| Error::internal_error())?;

        *value = status_value;

        Ok(())
    }

    pub fn router(&self) -> Router {
        Router::new().route(
            "/",
            get({
                let this = self.clone();

                || async move { Json(this.get_value().await) }
            }),
        )
    }
}
