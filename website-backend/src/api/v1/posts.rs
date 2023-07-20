use crate::{data::post::Post, error::Error, Service};
use axum::{extract::State, routing::get, Json, Router};
use sqlx::query_as;
use tracing::instrument;

/// A dummy endpoint for posts.
///
/// This may become a listing endpoint in the future for let's say.. job posts.
#[instrument(skip(service))]
async fn posts(State(service): State<Service>) -> Result<Json<Vec<Post>>, Error> {
    // Auth, db fetch, all bells and whistles..

    // Just an example of a db fetch.
    // Not an architectural suggestion.
    let db = service.db();
    let posts = query_as!(
        Post,
        r#"
SELECT writer, content
FROM posts
ORDER BY writer ASC
        "#
    )
    .fetch_all(&*db)
    .await?;

    Ok(Json(posts))
}

/// Posts routes.
pub fn routes() -> Router<Service> {
    Router::new().route("/", get(posts))
}
