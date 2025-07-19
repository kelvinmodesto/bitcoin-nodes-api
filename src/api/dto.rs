use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAPI {
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub alias: String,
    pub channels: u32,
    pub capacity: u64,
    #[serde(rename = "firstSeen")]
    pub first_seen: u64,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    pub city: Option<HashMap<String, String>>,
    pub country: HashMap<String, String>,
    pub iso_code: String,
    #[serde(default, deserialize_with = "deserialize_json_to_string_hashmap")]
    pub subdivision: Option<HashMap<String, String>>,
}

fn deserialize_json_to_string_hashmap<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(json_str) = opt {
        let map: HashMap<String, String> =
            serde_json::from_str(&json_str).map_err(serde::de::Error::custom)?;
        Ok(Some(map))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::generate_valid_node_api;

    #[test]
    fn test_node_with_all_fields() {
        let node = generate_valid_node_api();

        assert!(!node.public_key.is_empty());
        assert!(!node.alias.is_empty());
        assert!(node.capacity > 0);
        assert!(node.channels > 0);
        assert!(node.first_seen > 0);
        assert!(node.updated_at > 0);
        assert!(node.updated_at >= node.first_seen);
        assert!(node.city.is_some());
        assert!(!node.country.is_empty());
        assert!(!node.iso_code.is_empty());
        assert!(node.subdivision.is_some());
    }

    #[test]
    fn test_node_without_city() {
        let mut node = generate_valid_node_api();
        node.city = None;

        assert!(!node.public_key.is_empty());
        assert!(!node.alias.is_empty());
        assert!(node.capacity > 0);
        assert!(node.channels > 0);
        assert!(node.city.is_none());
        assert!(!node.country.is_empty());
        assert!(!node.iso_code.is_empty());
    }

    #[test]
    fn test_node_without_subdivision() {
        let mut node = generate_valid_node_api();
        node.subdivision = None;

        assert!(!node.public_key.is_empty());
        assert!(!node.alias.is_empty());
        assert!(node.capacity > 0);
        assert!(node.channels > 0);
        assert!(node.subdivision.is_none());
        assert!(node.city.is_some());
        assert!(!node.country.is_empty());
    }

    #[test]
    fn test_node_with_zero_channels() {
        let mut node = generate_valid_node_api();
        node.channels = 0;

        assert_eq!(node.channels, 0);
        assert!(node.capacity > 0);
    }
}
