#!/bin/sh
# Roda só na primeira inicialização do volume do Postgres.
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
  -c "CREATE DATABASE moang_dev OWNER $POSTGRES_USER"
