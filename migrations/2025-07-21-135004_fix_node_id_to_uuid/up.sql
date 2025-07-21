-- Drop the existing table and recreate with UUID
DROP TABLE nodes;

-- Create extension for UUID generation if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Recreate nodes table with UUID primary key
CREATE TABLE nodes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    public_key VARCHAR NOT NULL UNIQUE,
    alias VARCHAR NOT NULL,
    capacity VARCHAR NOT NULL,
    first_seen VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add trigger for updated_at
SELECT diesel_manage_updated_at('nodes');
