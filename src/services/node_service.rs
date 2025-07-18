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
    fn test_multiple_services_are_independent() {
        // Each service should have been independent
        assert!(!std::ptr::eq(
            &NodeService::new().client,
            &NodeService::new().client
        ));
    }

    #[test]
    fn test_node_service_is_send_sync() {
        fn assert_require_send<T: Send + Sync>() {}
        assert_require_send::<NodeService>();
    }
}
