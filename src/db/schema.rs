// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Uuid,
        public_key -> Varchar,
        alias -> Varchar,
        capacity -> Varchar,
        first_seen -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
