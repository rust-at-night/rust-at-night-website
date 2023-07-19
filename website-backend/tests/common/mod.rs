#![allow(dead_code)]

use axum::Router;
use clap::Parser;
use tokio::runtime::Runtime;
use website_backend_lib::{options::Options, Service};

pub fn rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to start tokio runtime.")
}

/// Initializes a router to use in tests.
pub async fn init_app() -> (Router, Options) {
    let options = Options::parse();

    // Add any additional prep..

    let service = Service::from_options(&options).unwrap();
    (service.router(), options)
}
