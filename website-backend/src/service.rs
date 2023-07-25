use crate::{
    api::{health, v1},
    db::migrate_and_connect_to_db,
    Options,
};
use axum::{
    body::{boxed, Body},
    extract::DefaultBodyLimit,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};
use camino::Utf8PathBuf;
use color_eyre::{eyre::Context, Result};
use hyper::{Request, StatusCode};
use sqlx::{Pool, Sqlite};
use std::{net::SocketAddr, sync::Arc};
use tokio::select;
use tower::ServiceExt;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{info, instrument};

/// The default body limit for the server.
///
/// It is currently set to 2MB. (Axum default)
const DEFAULT_BODY_LIMIT: usize = 2 * 1024 * 1024;

/// Holds the state of the service.
pub struct ServiceInner {
    options: Options,
    db: Arc<Pool<Sqlite>>,
    // Extend the state if necessary..
}

#[derive(Clone)]
pub struct Service(Arc<ServiceInner>);

impl std::ops::Deref for Service {
    type Target = ServiceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Service {
    /// Makes the service from options.
    pub async fn from_options(options: &Options) -> Result<Self> {
        Ok(Self(Arc::new(ServiceInner {
            options: options.clone(),
            db: Arc::new(
                migrate_and_connect_to_db(options)
                    .await
                    .context("Connecting to db..")?,
            ),
        })))
    }

    // This function will be updated..
    #[allow(clippy::unwrap_used)]
    /// Serves the frontend.
    ///
    /// This is just a boilerplate and will be improved..
    ///
    /// # Panics
    ///
    /// This function has panics because it is in development.
    /// Those will be removed.
    pub async fn serve_frontend(req: Request<Body>, static_dir: Utf8PathBuf) -> impl IntoResponse {
        match ServeDir::new(&static_dir).oneshot(req).await {
            Ok(res) => {
                let status = res.status();
                match status {
                    StatusCode::NOT_FOUND => {
                        let index_path = static_dir.join("index.html");
                        let index_content = match tokio::fs::read_to_string(index_path).await {
                            Err(_) => {
                                return Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(boxed(Body::from("Internal server error.")))
                                    .unwrap()
                            }
                            Ok(index_content) => index_content,
                        };

                        Response::builder()
                            .status(StatusCode::OK)
                            .body(boxed(Body::from(index_content)))
                            .unwrap()
                    }
                    _ => res.map(boxed),
                }
            }
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed(Body::from(format!("error: {err}"))))
                .expect("error response"),
        }
    }

    /// Initialize application router.
    pub fn router(&self) -> Router {
        let static_dir = self.options.static_dir.clone();
        Router::new()
            .route("/health", get(health))
            .nest("/api/v1", v1::routes())
            .fallback_service(get(|req| Self::serve_frontend(req, static_dir)))
            .layer(CorsLayer::very_permissive())
            .layer(TraceLayer::new_for_http())
            .layer(DefaultBodyLimit::max(DEFAULT_BODY_LIMIT))
            .with_state(self.clone())
    }

    /// Initialize API.
    #[instrument(skip(self, addr), ret, err)]
    pub async fn launch_api(&self, addr: SocketAddr) -> Result<()> {
        let app = self.router();
        info!("Launching server on {addr}");
        Server::bind(&addr).serve(app.into_make_service()).await?;
        Ok(())
    }

    /// Init all tasks and run.
    pub async fn run(&self) -> Result<()> {
        let Options { addr, .. } = self.options();
        let server = self.launch_api(*addr);

        // Await all tasks
        select! {
            result = server => result?,
            // If necessary you may add background tasks here..
        }

        Ok(())
    }

    #[must_use]
    pub fn options(&self) -> &Options {
        &self.options
    }

    #[must_use]
    pub fn db(&self) -> Arc<Pool<Sqlite>> {
        Arc::clone(&self.db)
    }
}
