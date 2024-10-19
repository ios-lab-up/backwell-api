#!/bin/bash

echo "Waiting for PostgreSQL to be ready..."

# Wait for PostgreSQL to be ready
until pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER"; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

echo "PostgreSQL is ready."

# Run migrations
echo "Running migrations..."
diesel migration run

# Start the application
echo "Starting the application..."
exec backwellApi
