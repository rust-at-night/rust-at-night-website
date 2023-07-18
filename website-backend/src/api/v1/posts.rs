use crate::Service;
use crate::{data::post::Post, error::Error};
use axum::{extract::State, routing::get, Json, Router};

use tracing::instrument;

/// A dummy endpoint for posts.
///
/// This may become a listing endpoint in the future for let's say.. job posts.
#[instrument(skip(_service))]
async fn posts(State(_service): State<Service>) -> Result<Json<Vec<Post>>, Error> {
    // Auth, db fetch, all bells and whistles..
    Ok(Json(vec![
        Post::new("ali", "I don't know what I'm doing!"),
        Post::new("ozan", "I'm good with namings!"),
        Post::new("caner", "I write a lot into the group!"),
    ]))
}

/// Posts routes.
pub fn routes() -> Router<Service> {
    Router::new().route("/", get(posts))
}
