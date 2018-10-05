#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER rocket_rc;
    CREATE DATABASE rocket_rc;
    GRANT ALL PRIVILEGES ON DATABASE rocket_rc TO rocket_rc;
EOSQL

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE rocket_rc_test;
    GRANT ALL PRIVILEGES ON DATABASE rocket_rc_test TO rocket_rc;
EOSQL
