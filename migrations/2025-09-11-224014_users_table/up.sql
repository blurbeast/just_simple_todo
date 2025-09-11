-- Your SQL goes here


CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    alias VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);