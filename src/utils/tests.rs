#[cfg(test)]
use crate::api::dto::NodeAPI;
use std::collections::HashMap;

#[cfg(test)]
fn parse_hashmap_from_json(json_str: &str) -> Option<HashMap<String, String>> {
    serde_json::from_str(json_str).ok()
}

#[cfg(test)]
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

#[cfg(test)]
pub fn generate_valid_node_api() -> NodeAPI {
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
