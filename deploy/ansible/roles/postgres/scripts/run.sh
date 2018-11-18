#!/bin/bash
docker network create rocket_net

docker run -d \
  --name rocket_rc_pg \
  --restart always \
  --network rocket_net \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -v /pg_data:/var/lib/postgresql/data \
  rocket_rc_pg

# Wait for postgres
export PGPASSWORD="$POSTGRES_PASSWORD"
until psql -h localhost -p 5432 -U postgres  -c '\l'; do
  echo "$(date +%Y%m%dt%H%M%S) Postgres is unavailable - sleeping" | logger
  sleep 1
done
echo "$(date +%Y%m%dt%H%M%S) Postgres is up - executing command" | logger

export DATABASE_URL="postgres://postgres:${POSTGRES_PASSWORD}@rocket_rc_pg:5432/rocket_rc"

# Migration
docker run \
  -e DATABASE_URL="$DATABASE_URL" \
  --network=rocket_net \
  diesel_cli \
  diesel database reset


