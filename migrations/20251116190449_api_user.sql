-- Add migration script here
create table api_users(
    api_key text primary key,
    user_agent text not null,
    created_at timestamptz not null default now()
)