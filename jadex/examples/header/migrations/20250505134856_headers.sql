-- Add migration script here
CREATE TABLE IF NOT EXISTS headers (
    id SERIAL PRIMARY KEY,
    hash TEXT NOT NULL
);