// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Int4,
        public_key -> Varchar,
        alias -> Varchar,
        capacity -> Varchar,
        first_seen -> Varchar,
    }
}
