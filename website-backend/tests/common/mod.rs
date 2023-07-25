#![allow(dead_code)]

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
pub async fn init_app() -> (Service, Options) {
    let mut options = Options::parse();
    options.database_url = "sqlite::memory:".to_string();

    let service = Service::from_options(&options).await.unwrap();

    // Apply project migrations.
    sqlx::migrate!("./migrations")
        .run(&*service.db())
        .await
        .unwrap();

    // Apply seeding.
    // TODO: Make this optional later on.
    sqlx::query(&std::fs::read_to_string("tests/data/db/tests_base_seed.sql").unwrap())
        .execute(&*service.db())
        .await
        .unwrap();

    // Add any additional prep..

    (service, options)
}
