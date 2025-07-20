use crate::api::dto::NodeAPI;
use chrono::DateTime;

#[derive(Debug, Clone)]
pub struct NodeDomain {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}

impl From<NodeAPI> for NodeDomain {
    fn from(api: NodeAPI) -> Self {
        Self {
            public_key: api.public_key,
            alias: api.alias,
            capacity: format_capacity(api.capacity),
            first_seen: format_timestamp_to_iso8601(api.first_seen),
        }
    }
}
fn format_timestamp_to_iso8601(timestamp: u64) -> String {
    match DateTime::from_timestamp(timestamp as i64, 0) {
        Some(datetime) => datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string(), // format: "2025-07-19T00:00:00Z"
        None => format!("Invalid timestamp: {}", timestamp),
    }
}

fn format_capacity(sats: u64) -> String {
    const SATS_PER_BTC: f64 = 100_000_000.0; // 1 BTC = 100.000.000 sats
    let amount = sats as f64 / SATS_PER_BTC;

    let formatted = format!("{:.8}", amount);

    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::generate_valid_node_api;

    #[test]
    fn test_node_api_to_domain_conversion() {
        let api_node = generate_valid_node_api();
        let domain_node: NodeDomain = api_node.into();

        assert_eq!(
            domain_node.public_key,
            "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
        );
        assert_eq!(domain_node.alias, "ACINQ");
        assert_eq!(domain_node.capacity, "1.5");

        // first_seen (1609459200) should be formatted as ISO 8601
        assert_eq!(domain_node.first_seen, "2021-01-01T00:00:00Z");
    }

    #[test]
    fn test_node_api_to_domain_with_different_values() {
        let mut api_node = generate_valid_node_api();
        api_node.capacity = 999999999;
        api_node.first_seen = 1234567890; // 2009-02-13T23:31:30Z
        api_node.alias = "Test Node".to_string();

        let domain_node: NodeDomain = api_node.into();

        assert_eq!(domain_node.capacity, "9.99999999");
        assert_eq!(domain_node.first_seen, "2009-02-13T23:31:30Z");
        assert_eq!(domain_node.alias, "Test Node");
    }

    #[test]
    fn test_timestamp_formatting_iso8601() {
        // Test the formatting function directly
        let formatted = format_timestamp_to_iso8601(1609459200);
        assert_eq!(formatted, "2021-01-01T00:00:00Z");

        let formatted2 = format_timestamp_to_iso8601(1704067200);
        assert_eq!(formatted2, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_invalid_timestamp_handling() {
        // Test with an invalid timestamp
        let formatted = format_timestamp_to_iso8601(i64::MAX as u64);
        assert!(formatted.starts_with("Invalid timestamp:"));
    }

    #[test]
    fn test_iso8601_format_validation() {
        let formatted = format_timestamp_to_iso8601(1609459200);

        // Verify it's valid ISO 8601 format
        assert!(formatted.contains("T"));
        assert!(formatted.ends_with("Z"));
        assert!(formatted.len() == 20); // "2021-01-01T00:00:00Z".len() == 20
    }
}
