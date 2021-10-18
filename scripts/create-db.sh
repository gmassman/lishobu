#!/usr/bin/env bash

PG_CONN="${PG_CONN:-postgres://postgres:postgres@localhost:5432}"
DB_NAME="${DB_NAME:-dev_lishobu}"

psql -d $PG_CONN <<SQL
SELECT 'CREATE DATABASE $DB_NAME'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$DB_NAME')\gexec
SQL
