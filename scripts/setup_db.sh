#!/bin/bash

# Create PostgreSQL database for development
# Run this script if you want to set up the database manually without Docker

set -e

DB_USER=${1:-postgres}
DB_PASSWORD=${2:-password}
DB_NAME=${3:-moneymanager}

echo "Setting up PostgreSQL database for development..."
echo "User: $DB_USER"
echo "Database: $DB_NAME"

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "PostgreSQL command line tool not found. Please install PostgreSQL."
    exit 1
fi

# Create database
echo "Creating database '$DB_NAME'..."
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -c "DROP DATABASE IF EXISTS $DB_NAME;"
PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -c "CREATE DATABASE $DB_NAME;"

echo "Database setup completed successfully."
echo "You can now run the application with:"
echo "  cargo run"

echo "Alternatively, you can use Docker Compose:"
echo "  docker-compose up"
