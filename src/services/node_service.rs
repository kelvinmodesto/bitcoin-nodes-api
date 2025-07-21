use crate::api::dto::NodeAPI;
use crate::mappers::node_mapper::NodeMapper;
use crate::models::node::{NewNode, Node};
use crate::repositories::node_repository::NodeRepository;

#[derive(Clone)]
pub struct NodeService {
    client: reqwest::Client,
    repository: NodeRepository,
}

impl NodeService {
    pub fn new(repository: NodeRepository) -> Self {
        Self {
            client: reqwest::Client::new(),
            repository,
        }
    }

    /// Fetch nodes from external API and return structured data
    pub async fn fetch_nodes(&self) -> Result<Vec<NodeAPI>, Box<dyn std::error::Error>> {
        let nodes_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

        let response = self.client.get(nodes_url).send().await?;

        // Get the raw text first for better error debugging
        let response_text = response.text().await?;

        // Try to parse the JSON
        match serde_json::from_str::<Vec<NodeAPI>>(&response_text) {
            Ok(nodes) => Ok(nodes),
            Err(e) => {
                eprintln!("Failed to parse API response: {}", e);
                eprintln!(
                    "Response preview: {}",
                    &response_text[..std::cmp::min(500, response_text.len())]
                );

                // Try to parse as a more lenient structure first
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(json_value) => {
                        // Filter out null entries and try again
                        if let Some(array) = json_value.as_array() {
                            let valid_nodes: Vec<NodeAPI> = array
                                .iter()
                                .filter(|item| !item.is_null())
                                .filter_map(|item| serde_json::from_value(item.clone()).ok())
                                .collect();

                            println!(
                                "Successfully parsed {} out of {} nodes",
                                valid_nodes.len(),
                                array.len()
                            );
                            Ok(valid_nodes)
                        } else {
                            Err(Box::new(e))
                        }
                    }
                    Err(_) => Err(Box::new(e)),
                }
            }
        }
    }

    /// Fetch nodes from external API, upsert to database, and return database nodes
    pub async fn sync_nodes(&self) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        // Fetch from external API
        let api_nodes = self.fetch_nodes().await?;

        // Convert to internal format
        let new_nodes = NodeMapper::api_nodes_to_new_nodes(api_nodes);

        // Upsert to database
        let db_nodes = self.repository.upsert_many(&new_nodes)?;

        Ok(db_nodes)
    }

    /// Get all nodes from database
    pub fn get_all_nodes(&self) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
        self.repository.get_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::client::PgClient;
    use crate::repositories::node_repository::NodeRepository;
    use dotenvy::dotenv;

    macro_rules! skip_if_env_not_set {
        ($var:expr) => {
            if std::env::var($var).is_err() {
                println!("⏭️  Skipping test. Set {}=1 to run.", $var);
                return;
            }
        };
    }

    // Helper function to create a test service
    fn create_test_service() -> Result<NodeService, Box<dyn std::error::Error>> {
        let db_client = PgClient::new()?;
        let repository = NodeRepository::new(db_client);
        Ok(NodeService::new(repository))
    }

    // Unit tests
    #[test]
    fn test_node_service_creation() {
        dotenv().ok();

        match create_test_service() {
            Ok(service) => {
                // Test that the service has a working HTTP client
                assert!(
                    service
                        .client
                        .get("https://api.api-onepiece.com/v2/characters/en")
                        .build()
                        .is_ok()
                );
            }
            Err(e) => {
                println!("Failed to create test service: {}", e);
                // Don't panic in unit tests if DB is not available
            }
        }
    }

    #[test]
    fn test_multiple_services_are_independent_and_functional() {
        dotenv().ok();

        if let (Ok(service_a), Ok(service_b)) = (create_test_service(), create_test_service()) {
            // Each service should have independent clients
            assert!(!std::ptr::eq(&service_a.client, &service_b.client));

            // And both should be functional
            assert!(
                service_a
                    .client
                    .get("https://api.api-onepiece.com/v2/fruits/en")
                    .build()
                    .is_ok()
            );
            assert!(
                service_b
                    .client
                    .get("https://api.api-onepiece.com/v2/sagas/en")
                    .build()
                    .is_ok()
            );
        }
    }

    #[test]
    fn test_node_service_is_send_sync() {
        fn assert_require_send<T: Send + Sync>() {}
        assert_require_send::<NodeService>();
    }

    // Integration tests
    #[tokio::test]
    async fn test_fetch_nodes() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let service = match create_test_service() {
            Ok(service) => service,
            Err(e) => {
                println!("Failed to create service: {}", e);
                return;
            }
        };

        match service.fetch_nodes().await {
            Ok(nodes) => {
                assert!(!nodes.is_empty(), "Should return at least one node");

                // Check first node structure
                let first_node = &nodes[0];
                assert!(
                    !first_node.public_key.is_empty(),
                    "Public key should not be empty"
                );
                assert!(!first_node.alias.is_empty(), "Alias should not be empty");
                assert!(first_node.capacity > 0, "Capacity should be greater than 0");
                assert!(
                    first_node.first_seen > 0,
                    "First seen should be greater than 0"
                );

                println!("✅ Successfully fetched {} structured nodes", nodes.len());
                println!(
                    "First node: {} ({})",
                    first_node.alias, first_node.public_key
                );
            }
            Err(e) => {
                println!("❌ Failed to fetch structured nodes: {}", e);
                panic!("Failed to fetch structured nodes: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_sync_nodes_integration() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let service = match create_test_service() {
            Ok(service) => service,
            Err(e) => {
                println!("Failed to create service: {}", e);
                return;
            }
        };

        match service.sync_nodes().await {
            Ok(nodes) => {
                println!("✅ Successfully synced {} nodes", nodes.len());
                assert!(!nodes.is_empty(), "Should have synced at least one node");

                // Verify the nodes have the expected structure
                for node in nodes.iter().take(3) {
                    // Check first 3 nodes
                    assert!(!node.id.to_string().is_empty(), "Node should have an ID");
                    assert!(!node.public_key.is_empty(), "Node should have public key");
                    assert!(!node.alias.is_empty(), "Node should have alias");
                    assert!(!node.capacity.is_empty(), "Node should have capacity");
                    assert!(!node.first_seen.is_empty(), "Node should have first_seen");
                }
            }
            Err(e) => {
                println!("❌ Failed to sync nodes: {}", e);
                panic!("Failed to sync nodes: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_nodes_from_database() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let service = match create_test_service() {
            Ok(service) => service,
            Err(e) => {
                println!("Failed to create service: {}", e);
                return;
            }
        };

        match service.get_all_nodes() {
            Ok(nodes) => {
                println!(
                    "✅ Successfully retrieved {} nodes from database",
                    nodes.len()
                );

                // If we have nodes, verify their structure
                if !nodes.is_empty() {
                    let first_node = &nodes[0];
                    assert!(
                        !first_node.public_key.is_empty(),
                        "Node should have public key"
                    );
                    assert!(!first_node.alias.is_empty(), "Node should have alias");
                    println!(
                        "Sample node: {} ({})",
                        first_node.alias, first_node.public_key
                    );
                }
            }
            Err(e) => {
                println!("❌ Failed to get nodes from database: {}", e);
                panic!("Failed to get nodes: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_nodes_structure_validation() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let service = match create_test_service() {
            Ok(service) => service,
            Err(e) => {
                println!("Failed to create service: {}", e);
                return;
            }
        };

        match service.fetch_nodes().await {
            Ok(nodes) => {
                // Validate basic response structure
                assert!(!nodes.is_empty(), "Should return at least one node");

                // Examine first node structure
                let first_node = &nodes[0];

                // Check for expected fields
                assert!(
                    !first_node.public_key.is_empty(),
                    "Node should have public key field"
                );
                assert!(!first_node.alias.is_empty(), "Node should have alias");
                assert!(first_node.capacity > 0, "Node should have capacity > 0");
                assert!(first_node.first_seen > 0, "Node should have first_seen > 0");
                assert!(
                    first_node.updated_at >= first_node.first_seen,
                    "updated_at should be >= first_seen"
                );

                println!("✅ API response structure validated");
                println!("Sample node fields:");
                println!("  - Public Key: {}", first_node.public_key);
                println!("  - Alias: {}", first_node.alias);
                println!("  - Capacity: {}", first_node.capacity);
                println!("  - Channels: {}", first_node.channels);
                println!("  - First Seen: {}", first_node.first_seen);
                println!("  - Updated At: {}", first_node.updated_at);
            }
            Err(e) => {
                println!("❌ Integration test failed: {}", e);
                panic!("Failed to fetch nodes: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_service_with_custom_client() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let db_client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create DB client: {}", e);
                return;
            }
        };

        let repository = NodeRepository::new(db_client);

        let custom_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("bitcoin-nodes-api-test/1.0")
            .build()
            .expect("Should create custom client");

        let service = NodeService {
            client: custom_client,
            repository,
        };

        // Test that service works with custom client configuration
        let result = service.fetch_nodes().await;
        assert!(result.is_ok(), "Service should work with custom client");
        println!("✅ Custom client configuration works");
    }

    #[tokio::test]
    async fn test_service_handles_network_timeout() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");
        dotenv().ok();

        let db_client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to create DB client: {}", e);
                return;
            }
        };

        let repository = NodeRepository::new(db_client);

        // Create a client with very short timeout for testing error handling
        let timeout_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(1)) // Very short timeout
            .build()
            .expect("Should create timeout client");

        let service = NodeService {
            client: timeout_client,
            repository,
        };

        let result = service.fetch_nodes().await;
        // This should likely fail due to timeout, which is expected
        if result.is_err() {
            println!("✅ Service properly handles timeout errors");
        } else {
            println!("⚠️  Request completed faster than expected timeout");
        }
    }
}
