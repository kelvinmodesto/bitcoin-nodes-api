use crate::db::schema::nodes;
use crate::models::node::Node;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool, PoolError, PooledConnection};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PgClient {
    pool: DbPool,
}

impl PgClient {
    /// Create a new PgClient with a connection pool
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(10).build(manager)?;

        Ok(PgClient { pool })
    }

    /// Create a new PgClient from an existing pool
    pub fn from_pool(pool: DbPool) -> Self {
        PgClient { pool }
    }

    /// Get a connection from the pool
    fn get_connection(&self) -> Result<DbConnection, PoolError> {
        self.pool.get()
    }

    /// Get all nodes from the database
    pub fn get_all_nodes(&self) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;

        let nodes = nodes::table.select(Node::as_select()).load(&mut conn)?;

        Ok(nodes)
    }

    /// Get the underlying connection pool
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get_all_nodes() {
        dotenv().ok();

        // This test requires a running database
        match PgClient::new() {
            Ok(client) => match client.get_all_nodes() {
                Ok(nodes) => {
                    println!("Found {} nodes", nodes.len());
                    for node in nodes {
                        println!("Node: {} - {}", node.alias, node.public_key);
                    }
                }
                Err(e) => println!("Error fetching nodes: {}", e),
            },
            Err(e) => println!("Failed to create client: {}", e),
        }
    }
}
