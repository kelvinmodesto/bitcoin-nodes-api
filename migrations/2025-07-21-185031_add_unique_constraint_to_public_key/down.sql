-- Remove unique constraint from public_key
ALTER TABLE nodes DROP CONSTRAINT nodes_public_key_unique;
