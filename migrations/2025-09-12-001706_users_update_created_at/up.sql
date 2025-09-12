-- Your SQL goes here

ALTER TABLE users
ALTER COLUMN created_at TYPE VARCHAR USING created_at::text;
