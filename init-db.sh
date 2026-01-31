#!/bin/bash
set -e

# Create the Keycloak database if it doesn't exist
#!/bin/bash
set -e
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "postgres" <<-EOSQL
    CREATE DATABASE "$KC_DB_NAME";
EOSQL