# FinGest (Rust)

This is a Rust implementation of the FinGest API that replicates the functionality of the original Java Spring Boot application.

## Tech Stack

- Rust 1.75.0+
- Actix-web 4.x (HTTP server)
- SQLx with PostgreSQL (Database)
- Tokio (Async runtime)
- Docker & Docker Compose for containerization

## Project Structure

```
money-manager-api-rs/
├── migrations/         # Database migration scripts
├── src/                # Source code
│   ├── api/            # API routes and handlers
│   ├── config/         # Configuration management
│   ├── db/             # Database access and models
│   ├── errors/         # Error handling
│   ├── models/         # Data models
│   ├── services/       # Business logic
│   └── utils/          # Utility functions
├── tests/              # Integration tests
├── Cargo.toml          # Project dependencies
├── Dockerfile          # Container definition
└── docker-compose.yml  # Local development setup
```

## Development Setup

### Prerequisites

- Rust 1.75.0 or later
- Docker and Docker Compose
- PostgreSQL 15+

### Setup Environment

1. Clone the repository
2. Copy the sample environment file:

```bash
cp sample.env .env
```

3. Configure the environment variables in `.env` as needed

### Running Locally

#### Using Cargo

1. Start PostgreSQL:

```bash
docker compose up db -d
```

2. Run the application:

```bash
cargo run
```

#### Using Docker Compose

```bash
docker compose up
```

### Database Migrations
```bash
docker compose up
```

### Database Migrations

Migrations are handled automatically when the application starts. The migration scripts are located in the `migrations` directory.

## API Endpoints

### Category Endpoints

- `GET /resources/categories` - Get all categories

### User Endpoints

- `GET /resources/users` - Get all users
- `PUT /resources/users/{login}?field=fieldName` - Update user field

### Wallet Endpoints

- `GET /resources/users/{login}/wallets` - Get wallets for a user
- `POST /resources/users/{login}/wallets` - Create a new wallet
- `GET /resources/users/{login}/wallets/{id}/summary` - Get wallet summary
- `GET /resources/users/{login}/wallets/{id}/expenses` - Get wallet expenses
- `POST /resources/users/{login}/wallets/{id}/expenses` - Add expense to wallet
- `DELETE /resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}` - Delete an expense
- `GET /resources/users/{login}/wallets/{id}/highest_expense` - Get highest expense
- `GET /resources/users/{login}/wallets/{id}/counted_categories` - Get expenses by category

### Budget Endpoints

- `GET /resources/users/{login}/budgets` - Get all budgets for user
- `POST /resources/users/{login}/budgets` - Create a new budget

## Testing

Run tests with:

```bash
cargo test
```

## Building for Production

```bash
cargo build --release
```

## Docker Build

```bash
docker build -t fingest-rs .
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
