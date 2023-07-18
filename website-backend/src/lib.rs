/// All api routes.
pub mod api;

/// All data models.
pub mod data;

/// All error types.
pub mod error;

/// Service configuration.
pub mod options;

/// Service itself.
pub mod service;

pub use error::Error;
pub use options::Options;
pub use service::Service;
