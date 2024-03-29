use crate::io::{Error, Result};
use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use super::KitRouter;

#[derive(Clone, Serialize, Default)]
pub enum StatusValue {
    #[default]
    Booting,
    Ready,
    Unhealthy,
    _ShuttingDown,
}

#[derive(Clone, Serialize, Default)]
pub struct StatusKit {
    value: Arc<Mutex<StatusValue>>,
}

impl StatusKit {
    pub fn new() -> Self {
        Default::default()
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
}

impl KitRouter for StatusKit {
    fn router(&self) -> Router {
        Router::new().route(
            "/",
            get({
                let this = self.clone();

                || async move { Json(this) }
            }),
        )
    }
}
