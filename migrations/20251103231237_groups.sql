-- Add migration script here
create table groups(
    id uuid primary key,
    name text unique not null,
    created_at timestamptz not null default now()
)