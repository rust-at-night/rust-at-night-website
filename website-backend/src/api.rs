pub mod v1;

/// Route for the health check.
pub async fn health() -> &'static str {
    "OK"
}
