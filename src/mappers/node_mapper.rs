use crate::api::dto::NodeAPI;
use crate::models::node::NewNode;
use chrono::DateTime;

pub struct NodeMapper;

impl NodeMapper {
    /// Convert external API node to internal NewNode
    pub fn api_to_new_node(api: NodeAPI) -> NewNode {
        NewNode {
            public_key: api.public_key,
            alias: api.alias,
            capacity: Self::format_capacity(api.capacity),
            first_seen: Self::format_timestamp_to_iso8601(api.first_seen),
        }
    }

    /// Convert multiple API nodes to NewNodes
    pub fn api_nodes_to_new_nodes(api_nodes: Vec<NodeAPI>) -> Vec<NewNode> {
        api_nodes.into_iter().map(Self::api_to_new_node).collect()
    }

    /// Format satoshi amount to BTC string
    fn format_capacity(sats: u64) -> String {
        const SATS_PER_BTC: f64 = 100_000_000.0; // 1 BTC = 100.000.000 sats
        let amount = sats as f64 / SATS_PER_BTC;

        let formatted = format!("{:.8}", amount);

        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    /// Format Unix timestamp to ISO 8601 string
    fn format_timestamp_to_iso8601(timestamp: u64) -> String {
        match DateTime::from_timestamp(timestamp as i64, 0) {
            Some(datetime) => datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            None => format!("Invalid timestamp: {}", timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::generate_valid_node_api;

    #[test]
    fn test_api_to_new_node_conversion() {
        let api_node = generate_valid_node_api();
        let new_node = NodeMapper::api_to_new_node(api_node);

        assert_eq!(
            new_node.public_key,
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
        );
        assert_eq!(new_node.alias, "ACINQ");
        assert_eq!(new_node.capacity, "1.5");
        assert_eq!(new_node.first_seen, "2021-01-01T00:00:00Z");
    }

    #[test]
    fn test_multiple_api_nodes_conversion() {
        let api_nodes = vec![generate_valid_node_api(), generate_valid_node_api()];

        let new_nodes = NodeMapper::api_nodes_to_new_nodes(api_nodes);

        assert_eq!(new_nodes.len(), 2);
        for node in new_nodes {
            assert!(!node.public_key.is_empty());
            assert!(!node.alias.is_empty());
        }
    }

    #[test]
    fn test_capacity_formatting() {
        assert_eq!(NodeMapper::format_capacity(100000000), "1");
        assert_eq!(NodeMapper::format_capacity(150000000), "1.5");
        assert_eq!(NodeMapper::format_capacity(50000000), "0.5");
        assert_eq!(NodeMapper::format_capacity(1000000), "0.01");
        assert_eq!(NodeMapper::format_capacity(1), "0.00000001");
        assert_eq!(NodeMapper::format_capacity(0), "0");
    }

    #[test]
    fn test_timestamp_formatting() {
        assert_eq!(
            NodeMapper::format_timestamp_to_iso8601(1609459200),
            "2021-01-01T00:00:00Z"
        );
        assert_eq!(
            NodeMapper::format_timestamp_to_iso8601(1704067200),
            "2024-01-01T00:00:00Z"
        );
    }

    #[test]
    fn test_invalid_timestamp() {
        let result = NodeMapper::format_timestamp_to_iso8601(i64::MAX as u64);
        assert!(result.starts_with("Invalid timestamp:"));
    }
}
