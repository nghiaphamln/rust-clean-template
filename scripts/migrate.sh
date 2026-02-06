#!/bin/bash
set -e

# Load environment variables from .env file if it exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

DATABASE_URL="${DATABASE_URL:-postgresql://postgres:postgres@localhost:5432/rust_clean_db}"

echo "Running Flyway migrations..."
echo "Database URL: $DATABASE_URL"

flyway -url="$DATABASE_URL" -locations=filesystem:./migrations migrate

echo "Migrations completed successfully!"
