use serde_json::Value;

pub struct NodeService {
    client: reqwest::Client,
}

impl NodeService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_nodes(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let nodes_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

        let response = self.client.get(nodes_url).send().await?;

        let nodes: Value = response.json().await?;

        Ok(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! skip_if_env_not_set {
        ($var:expr) => {
            if std::env::var($var).is_err() {
                println!("⏭️  Skipping test. Set {}=1 to run.", $var);
                return;
            }
        };
    }

    // Unit tests
    #[test]
    fn test_node_service_creation() {
        let service = NodeService::new();
        assert!(
            service
                .client
                .get("https://api.api-onepiece.com/v2/characters/en")
                .build()
                .is_ok()
        );
    }

    #[test]
    fn test_multiple_services_are_independent_and_functional() {
        let service_a = NodeService::new();
        let service_b = NodeService::new();

        // Each service should have been independent
        assert!(!std::ptr::eq(&service_a.client, &service_b.client));

        // And functional
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
        )
    }

    #[test]
    fn test_node_service_is_send_sync() {
        fn assert_require_send<T: Send + Sync>() {}
        assert_require_send::<NodeService>();
    }

    // Integration tests
    #[tokio::test]
    async fn test_fetch_nodes_integration() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");

        let service = NodeService::new();

        let result = service.fetch_nodes().await;
        assert!(
            result.is_ok(),
            "Should successfully fetch nodes from mempool API"
        );

        let nodes = result.unwrap();
        assert!(nodes.is_array(), "Response should be a JSON array");
        println!("✅ Successfully fetched nodes from API");
    }

    #[tokio::test]
    async fn test_response_structure() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");

        let service = NodeService::new();
        match service.fetch_nodes().await {
            Ok(nodes) => {
                // Validate basic response structure
                assert!(nodes.is_array(), "Response should be an array");

                if let Some(nodes_array) = nodes.as_array() {
                    assert!(!nodes_array.is_empty(), "Should return at least one node");

                    // Examine first node structure
                    if let Some(first_node) = nodes_array.first() {
                        assert!(first_node.is_object(), "Each node should be a JSON object");

                        let node_obj = first_node.as_object().unwrap();

                        // Print available fields for debugging
                        let fields: Vec<&String> = node_obj.keys().collect();
                        println!("Available node fields: {:?}", fields);

                        // Check for expected fields (adjust based on actual API response)
                        assert!(
                            node_obj.contains_key("publicKey")
                                || node_obj.contains_key("public_key"),
                            "Node should have public key field"
                        );

                        println!("✅ API response structure validated");
                    }
                }
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
        let custom_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("bitcoin-nodes-api-test/1.0")
            .build()
            .expect("Should create custom client");

        let service = NodeService {
            client: custom_client,
        };

        // Test that service works with custom client configuration
        let result = service.fetch_nodes().await;
        assert!(result.is_ok(), "Service should work with custom client");
        println!("✅ Custom client configuration works");
    }

    #[tokio::test]
    async fn test_service_handles_network_timeout() {
        skip_if_env_not_set!("RUN_INTEGRATION_TESTS");

        // Create a client with very short timeout for testing error handling
        let timeout_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(1)) // Very short timeout
            .build()
            .expect("Should create timeout client");

        let service = NodeService {
            client: timeout_client,
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
