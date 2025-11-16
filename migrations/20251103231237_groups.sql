-- Add migration script here
create table groups(
    vrc_group_id text primary key,
    name text unique not null,
    created_at timestamptz not null default now()
)