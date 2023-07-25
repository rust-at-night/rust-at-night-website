use crate::Options;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::time::Duration;

/// Maximum number of concurrent connections to the database.
///
/// Probably we'll use a t2.micro or small instance for the backend initially.
/// `20` should be a safe number for now.
const MAX_CONNECTIONS: u32 = 20;

/// Maximum lifetime of a connection to the database.
///
/// Infinite connections are not recommended due to the unfortunate reality of memory/resource leaks on the database-side.
/// It is better to retire connections periodically (even if only once daily) to allow the database
/// the opportunity to clean up data structures (parse trees, query metadata caches, thread-local storage, etc.)
/// that are associated with a session.
const MAX_CONNECTION_LIFETIME: Duration = Duration::from_secs(60 * 60 * 12); // 12 hours

pub async fn migrate_and_connect_to_db(options: &Options) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .max_lifetime(MAX_CONNECTION_LIFETIME)
        .connect(&options.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
