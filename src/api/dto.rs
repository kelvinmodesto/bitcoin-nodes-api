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
    use super::*;

    fn parse_hashmap_from_json(json_str: &str) -> Option<HashMap<String, String>> {
        serde_json::from_str(json_str).ok()
    }
    fn generate_mock_hash_map() -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();

        result.insert("de".to_string(), "Brooklyn".to_string());
        result.insert("en".to_string(), "Brooklyn".to_string());
        result.insert("es".to_string(), "Brooklyn".to_string());
        result.insert("fr".to_string(), "Brooklyn".to_string());
        result.insert("ja".to_string(), "ブルックリン区".to_string());
        result.insert("pt-BR".to_string(), "Brooklyn".to_string());
        result.insert("ru".to_string(), "Бруклин".to_string());
        result.insert("zh-CN".to_string(), "布鲁克林区".to_string());

        result
    }

    fn generate_valid_node() -> NodeAPI {
        let city: HashMap<String, String> = generate_mock_hash_map();
        let country: HashMap<String, String> = generate_mock_hash_map();

        let subdivision_json =
            r#"{"de":"Hessen","en":"Hesse","es":"Hessen","fr":"Hesse","ru":"Гессен"}"#;
        let subdivision = parse_hashmap_from_json(subdivision_json);

        NodeAPI {
            public_key: String::from(
                "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f",
            ),
            alias: String::from("ACINQ"),
            channels: 42,
            capacity: 150000000,
            first_seen: 1609459200,
            updated_at: 1704067200,
            city: Some(city),
            country,
            iso_code: String::from("CA"),
            subdivision,
        }
    }

    #[test]
    fn test_node_with_all_fields() {
        let node = generate_valid_node();

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
        let mut node = generate_valid_node();
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
        let mut node = generate_valid_node();
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
        let mut node = generate_valid_node();
        node.channels = 0;

        assert_eq!(node.channels, 0);
        assert!(node.capacity > 0);
    }
}
