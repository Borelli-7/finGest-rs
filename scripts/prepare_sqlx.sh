#!/usr/bin/env bash

# Create .env file from sample if it doesn't exist
if [ ! -f .env ]; then
  cp sample.env .env
  echo "Created .env file from sample.env"
  echo "Please update the database connection information in .env before continuing"
  exit 0
fi

# Load the environment variables
source .env

# Make sure SQLx CLI is installed
if ! command -v sqlx &> /dev/null; then
    echo "SQLx CLI not found, installing..."
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
fi

# Generate SQLx prepared statements for offline mode
echo "Generating SQLx data for offline compilation..."
cargo sqlx prepare --database-url "${DATABASE_URL}" -- --lib

echo "Done! You can now build the project without a database connection."
