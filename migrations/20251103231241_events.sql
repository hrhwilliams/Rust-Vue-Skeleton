-- Add migration script here
CREATE TABLE events(
    id uuid PRIMARY KEY,
    group_id uuid NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    starts_at timestamptz NOT NULL,
    ends_at timestamptz NOT NULL,
    CONSTRAINT check_event_dates CHECK (starts_at <= ends_at)
)