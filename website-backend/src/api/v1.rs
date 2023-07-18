pub mod posts;

use crate::Service;
use axum::Router;
use tower_http::cors::CorsLayer;

/// Currently includes a dummy endpoint for posts.
///
/// This is where we would add our own endpoints.
pub fn routes() -> Router<Service> {
    Router::new()
        .nest("/posts", posts::routes())
        .layer(CorsLayer::very_permissive())
}
