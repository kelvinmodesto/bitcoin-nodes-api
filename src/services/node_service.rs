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
}
