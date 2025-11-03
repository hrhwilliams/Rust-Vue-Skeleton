-- Add migration script here
CREATE TABLE groups(
    id uuid PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
)