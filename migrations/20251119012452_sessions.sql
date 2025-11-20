-- Add migration script here
create table sessions(
    id text primary key not null,
    expires timestamptz not null,
    created_at timestamptz not null default now(),
    store text not null default '{}'
)