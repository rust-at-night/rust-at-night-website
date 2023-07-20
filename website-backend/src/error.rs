use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use tracing::{event, Level};

// Error handling strategy will be built on top of this pattern but also re-iterated in the future.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    // Extend with error types and from implementations as needed..
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    DatabaseInteraction(#[from] sqlx::Error),
}

impl Error {
    #[allow(clippy::match_single_binding)]

    /// The default status code for all errors is `INTERNAL_SERVER_ERROR`.
    /// Override this method to provide a different status code for a different error type.
    const fn status(&self) -> StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Simply passes the internal error through.
    ///
    /// The information here could be reduced later on.
    fn response(&self) -> ErrorResponse {
        ErrorResponse {
            message: self.to_string(),
            causes: {
                let mut error = self as &dyn StdError;
                let mut causes = vec![];
                while let Some(cause) = error.source() {
                    causes.push(cause.to_string());
                    error = cause;
                }
                causes
            },
            error: format!("{self:?}"),
            status: self.status().as_u16(),
        }
    }

    #[allow(clippy::unused_self)]
    /// Determine if this error should result in a trace event.
    ///
    /// Match on the error type and return false if it should not be traced.
    const fn trace(&self) -> bool {
        true
    }
}

/// Error response.
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
    causes: Vec<String>,
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status();
        let response = self.response();
        if self.trace() {
            event!(Level::ERROR, message = format!("{self:#}"), debug = ?&self);
        }
        (status, Json(response)).into_response()
    }
}
