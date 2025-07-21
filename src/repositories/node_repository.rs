use crate::db::client::PgClient;
use crate::models::node::{NewNode, Node};

pub struct NodeRepository {
    db_client: PgClient,
}

impl NodeRepository {
    pub fn new(db_client: PgClient) -> Self {
        Self { db_client }
    }

    /// Get all nodes from the database
    pub fn get_all(&self) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        self.db_client.get_all_nodes()
    }

    /// Upsert a single node
    pub fn upsert(&self, new_node: &NewNode) -> Result<Node, Box<dyn std::error::Error>> {
        self.db_client.upsert_node(new_node)
    }

    /// Upsert multiple nodes
    pub fn upsert_many(
        &self,
        new_nodes: &[NewNode],
    ) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        self.db_client.upsert_nodes(new_nodes)
    }
}

impl Clone for NodeRepository {
    fn clone(&self) -> Self {
        Self {
            db_client: self.db_client.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_node_repository_integration() {
        dotenv().ok();

        let db_client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create client: {}", e);
                return;
            }
        };

        let repo = NodeRepository::new(db_client);

        match repo.get_all() {
            Ok(nodes) => println!("Repository found {} nodes", nodes.len()),
            Err(e) => println!("Repository error: {}", e),
        }
    }
}
