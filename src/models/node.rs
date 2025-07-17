#[derive(Debug, Clone)]
pub struct Node {
    public_key: String,
    alias: String,
    capacity: u64,
    channels: u32,
    first_seen: u64,
    updated_at: u64,
}

impl Node {
    pub fn new(
        public_key: String,
        alias: String,
        capacity: u64,
        channels: u32,
        first_seen: u64,
        updated_at: u64,
    ) -> Self {
        Self {
            public_key,
            alias,
            capacity,
            channels,
            first_seen,
            updated_at,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn generate_valid_node() -> Node {
        Node::new(
            String::from("03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"),
            String::from("ACINQ"),
            150000000,
            42,
            1609459200,
            1704067200,
        )
    }

    #[test]
    fn should_be_valid_node() {
        let valid_node = generate_valid_node();

        assert!(!valid_node.public_key.is_empty());
        assert!(!valid_node.alias.is_empty());
        assert!(valid_node.capacity > 0);
        assert!(valid_node.channels > 0);
        assert!(valid_node.first_seen > 0);
        assert!(valid_node.updated_at > 0);
        assert!(valid_node.updated_at >= valid_node.first_seen);
    }
}
