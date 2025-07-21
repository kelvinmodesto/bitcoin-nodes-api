pub mod client;
pub mod schema;

pub use client::{DbConnection, DbPool, PgClient};

// Re-export commonly used types
pub use diesel::prelude::*;

// Helper function to establish a connection pool
pub fn establish_connection_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    PgClient::new().map(|client| client.pool().clone())
}
