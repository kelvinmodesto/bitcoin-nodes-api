// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Int4,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        alias -> Varchar,
        #[max_length = 255]
        capacity -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        first_seen -> Varchar,
    }
}
