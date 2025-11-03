#!/bin/bash
source .env

CONTAINER_NAME="postgres_$(date '+%s')"

docker run \
    --env POSTGRES_USER=${DB_USER} \
    --env POSTGRES_PASSWORD=${DB_PASS} \
    --health-cmd="pg_isready -U ${DB_USER} || exit 1" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --publish "${DB_PORT}":5432 \
    --detach \
    --name "${CONTAINER_NAME}" \
    postgres:18-alpine -N 1000

export DATABASE_URL="postgres://${DB_NAME}:${DB_PASS}@localhost:${DB_PORT}/${DB_NAME}"
echo $DATABASE_URL

sqlx database create
sqlx migrate run
