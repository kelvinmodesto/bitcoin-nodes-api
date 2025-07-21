-- Add unique constraint to public_key if it doesn't exist
ALTER TABLE nodes
ADD CONSTRAINT nodes_public_key_unique UNIQUE (public_key);
