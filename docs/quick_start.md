# Quick Start Guide

This guide provides instructions on how to quickly get started with the FinGest API (Rust version).

## Prerequisites

- Rust 1.75.0 or later
- PostgreSQL 15 or later
- Docker and Docker Compose (optional, for containerized setup)

## Local Development Setup

### 1. Clone the Repository

If you haven't already, clone the repository:

```bash
git clone <repository-url>
cd Money-manager/money-manager-api-rs
```

### 2. Set Up the Environment

Create a `.env` file from the sample:

```bash
cp sample.env .env
```

Edit the `.env` file to match your local environment.

### 3. Set Up the Database

You have two options:

#### Option 1: Using Docker Compose

```bash
docker-compose up db -d
```

This starts a PostgreSQL container with the correct configuration.

#### Option 2: Using an Existing PostgreSQL Instance

Run the setup script:

```bash
./scripts/setup_db.sh [username] [password] [database_name]
```

Or manually create the database:

```bash
psql -U postgres -c "CREATE DATABASE moneymanager;"
```

### 4. Run the Application

#### Option 1: Using Cargo

```bash
cargo run
```

#### Option 2: Using Docker Compose

```bash
docker-compose up
```

#### Option 3: Using the Build Script

```bash
./scripts/build_and_run.sh
```

## Testing

Run the tests:

```bash
cargo test
```

## API Documentation

See the `docs/api_examples.md` file for examples of API requests using curl.

## Next Steps

1. Check the main `README.md` for more detailed information
2. Explore the codebase to understand the structure
3. Try making some API requests using the examples in the documentation
4. Create your first feature or bugfix
