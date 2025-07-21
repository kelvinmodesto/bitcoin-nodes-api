DROP TABLE IF EXISTS nodes;

CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    public_key VARCHAR(255) NOT NULL,
    alias VARCHAR(255) NOT NULL,
    capacity VARCHAR(255) NOT NULL,
    first_seen VARCHAR(255) NOT NULL
);
