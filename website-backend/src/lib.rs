// Most of the lints we deny here have a good chance to be relevant for our project.
#![deny(clippy::all)]
// We warn for all lints on the planet. Just to filter them later for customization.
// It is impossible to remember all the lints so a subtractive approach keeps us updated, in control and knowledgeable.
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, clippy::restriction)]
// Then in the end we allow ridiculous or too restrictive lints that are not relevant for our project.
// This list is dynamic and will grow in time which will define our style.
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_docs_in_private_items,
    clippy::pub_use,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::implicit_return,
    clippy::missing_inline_in_public_items,
    clippy::similar_names,
    clippy::question_mark_used,
    clippy::expect_used,
    clippy::missing_errors_doc,
    clippy::pattern_type_mismatch,
    clippy::module_name_repetitions,
    clippy::empty_structs_with_brackets,
    clippy::as_conversions,
    clippy::self_named_module_files,
    clippy::cargo_common_metadata,
    clippy::exhaustive_structs
)]

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

/// Database connection.
pub mod db;

pub use error::Error;
pub use options::Options;
pub use service::Service;
