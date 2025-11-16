-- Add migration script here
create table events(
    id uuid primary key,
    group_id text not null references groups(id) on delete cascade,
    vrc_event_id text not null,
    name text unique not null,
    description text not null,
    starts_at timestamptz not null,
    ends_at timestamptz not null,
    category text not null,
    access_type text not null,
    platforms text[] not null,
    image_url text,
    tags text[],
    created_at timestamptz not null default now()
    constraint check_event_dates check (starts_at <= ends_at)
)