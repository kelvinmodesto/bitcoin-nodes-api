use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = crate::db::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    pub id: i32,
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Clone, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::nodes)]
pub struct NewNode {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node_creation() {
        let new_node = NewNode {
            public_key: "test_key".to_string(),
            alias: "Test Node".to_string(),
            capacity: "1.5".to_string(),
            first_seen: "2021-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(new_node.public_key, "test_key");
        assert_eq!(new_node.alias, "Test Node");
        assert_eq!(new_node.capacity, "1.5");
        assert_eq!(new_node.first_seen, "2021-01-01T00:00:00Z");
    }
}
