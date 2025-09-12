-- Your SQL goes here

ALTER TABLE todos
ALTER COLUMN created_at TYPE VARCHAR USING created_at::text;