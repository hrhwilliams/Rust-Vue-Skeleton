-- Add migration script here
create table events(
    id uuid primary key,
    group_id uuid not null references groups(id) on delete cascade,
    name text not null,
    description text not null,
    starts_at timestamptz not null,
    ends_at timestamptz not null,
    created_at timestamptz not null default now()
    constraint check_event_dates check (starts_at <= ends_at)
)