CREATE TABLE nodes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_key VARCHAR NOT NULL UNIQUE,
    alias VARCHAR NOT NULL,
    capacity VARCHAR NOT NULL,
    first_seen VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
