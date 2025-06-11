#!/bin/bash

# Build and run the Money Manager API Rust application

set -e

echo "Building Money Manager API (Rust)..."

# Build the application
cargo build

echo "Starting the application..."
echo "Make sure you have set up the database and configured the .env file"

# Run the application
cargo run
