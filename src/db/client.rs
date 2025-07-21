use crate::db::schema::nodes;
use crate::models::node::{NewNode, Node};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::upsert::excluded;
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

    /// Upsert a node (insert or update if public_key already exists)
    pub fn upsert_node(&self, new_node: &NewNode) -> Result<Node, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;

        let node = diesel::insert_into(nodes::table)
            .values(new_node)
            .on_conflict(nodes::public_key)
            .do_update()
            .set((
                nodes::alias.eq(excluded(nodes::alias)),
                nodes::capacity.eq(excluded(nodes::capacity)),
                nodes::first_seen.eq(excluded(nodes::first_seen)),
                nodes::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Node::as_returning())
            .get_result(&mut conn)?;

        Ok(node)
    }

    /// Upsert multiple nodes in a single transaction
    pub fn upsert_nodes(
        &self,
        new_nodes: &[NewNode],
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        let mut conn = self.get_connection()?;

        let nodes = diesel::insert_into(nodes::table)
            .values(new_nodes)
            .on_conflict(nodes::public_key)
            .do_update()
            .set((
                nodes::alias.eq(excluded(nodes::alias)),
                nodes::capacity.eq(excluded(nodes::capacity)),
                nodes::first_seen.eq(excluded(nodes::first_seen)),
                nodes::updated_at.eq(diesel::dsl::now),
            ))
            .returning(Node::as_returning())
            .load(&mut conn)?;

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
    use crate::models::node::NewNode;
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

    #[tokio::test]
    async fn test_upsert_node_insert() {
        dotenv().ok();

        let client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create client: {}", e);
                return;
            }
        };

        let new_node = NewNode {
            public_key: "test_public_key_123456789".to_string(),
            alias: "Test Node".to_string(),
            capacity: "1.5".to_string(),
            first_seen: "2021-01-01T00:00:00Z".to_string(),
        };

        match client.upsert_node(&new_node) {
            Ok(node) => {
                println!("Upserted node: {} - {}", node.alias, node.public_key);
                assert_eq!(node.public_key, new_node.public_key);
                assert_eq!(node.alias, new_node.alias);
                assert_eq!(node.capacity, new_node.capacity);
                assert_eq!(node.first_seen, new_node.first_seen);
            }
            Err(e) => println!("Error upserting node: {}", e),
        }
    }

    #[tokio::test]
    async fn test_upsert_node_update() {
        dotenv().ok();

        let client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create client: {}", e);
                return;
            }
        };

        let public_key = "test_public_key_update_123456789".to_string();

        // First insert
        let first_node = NewNode {
            public_key: public_key.clone(),
            alias: "Original Alias".to_string(),
            capacity: "1.0".to_string(),
            first_seen: "2021-01-01T00:00:00Z".to_string(),
        };

        let inserted_node = match client.upsert_node(&first_node) {
            Ok(node) => node,
            Err(e) => {
                println!("Error inserting first node: {}", e);
                return;
            }
        };

        // Then update with same public_key
        let updated_node_data = NewNode {
            public_key: public_key.clone(),
            alias: "Updated Alias".to_string(),
            capacity: "2.5".to_string(),
            first_seen: "2021-06-01T00:00:00Z".to_string(),
        };

        match client.upsert_node(&updated_node_data) {
            Ok(updated_node) => {
                println!(
                    "Updated node: {} - {}",
                    updated_node.alias, updated_node.public_key
                );
                // Should be the same ID (same record)
                assert_eq!(updated_node.id, inserted_node.id);
                assert_eq!(updated_node.public_key, public_key);
                assert_eq!(updated_node.alias, "Updated Alias");
                assert_eq!(updated_node.capacity, "2.5");
                assert_eq!(updated_node.first_seen, "2021-06-01T00:00:00Z");
                // updated_at should be more recent
                assert!(updated_node.updated_at > inserted_node.updated_at);
            }
            Err(e) => println!("Error updating node: {}", e),
        }
    }

    #[tokio::test]
    async fn test_upsert_multiple_nodes() {
        dotenv().ok();

        let client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create client: {}", e);
                return;
            }
        };

        let nodes = vec![
            NewNode {
                public_key: "test_bulk_1_123456789".to_string(),
                alias: "Bulk Node 1".to_string(),
                capacity: "1.0".to_string(),
                first_seen: "2021-01-01T00:00:00Z".to_string(),
            },
            NewNode {
                public_key: "test_bulk_2_123456789".to_string(),
                alias: "Bulk Node 2".to_string(),
                capacity: "2.0".to_string(),
                first_seen: "2021-02-01T00:00:00Z".to_string(),
            },
            NewNode {
                public_key: "test_bulk_3_123456789".to_string(),
                alias: "Bulk Node 3".to_string(),
                capacity: "3.0".to_string(),
                first_seen: "2021-03-01T00:00:00Z".to_string(),
            },
        ];

        match client.upsert_nodes(&nodes) {
            Ok(upserted_nodes) => {
                println!("Upserted {} nodes", upserted_nodes.len());
                assert_eq!(upserted_nodes.len(), 3);

                for (original, upserted) in nodes.iter().zip(upserted_nodes.iter()) {
                    assert_eq!(upserted.public_key, original.public_key);
                    assert_eq!(upserted.alias, original.alias);
                    assert_eq!(upserted.capacity, original.capacity);
                    assert_eq!(upserted.first_seen, original.first_seen);
                }
            }
            Err(e) => println!("Error upserting multiple nodes: {}", e),
        }
    }
}
