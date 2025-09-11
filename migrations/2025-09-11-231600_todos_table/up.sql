-- Your SQL goes here


CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR(1000),
    public_id UUID NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    completed BOOLEAN NOT NULL DEFAULT FALSE
);