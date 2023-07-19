// Axum wants handlers to be async, so we don't need this lint in this module.
#![allow(clippy::unused_async)]

pub mod v1;

/// Route for the health check.
pub async fn health() -> &'static str {
    "OK"
}
