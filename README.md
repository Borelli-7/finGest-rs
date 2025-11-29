# FinGest (Rust)

> A high-performance personal finance management API built with Rust

FinGest is a RESTful API server for managing personal finances, including wallets, expenses, budgets, and savings goals. This Rust implementation provides robust authentication, comprehensive financial tracking, and a secure, type-safe architecture.

## Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Environment Setup](#environment-setup)
  - [Running the Application](#running-the-application)
- [Authentication](#authentication)
- [API Documentation](#api-documentation)
- [Database Schema](#database-schema)
- [Development](#development)
- [Testing](#testing)
  - [Unit and Integration Tests](#unit-and-integration-tests)
  - [Postman Collection Testing](#postman-collection-testing)
  - [Apache JMeter Performance Testing](#apache-jmeter-performance-testing)
- [Docker Deployment](#docker-deployment)
- [Contributing](#contributing)
- [License](#license)

## Features

- **JWT-based Authentication**: Secure user registration and login with bcrypt password hashing
- **Wallet Management**: Create and manage multiple wallets with different currencies
- **Expense Tracking**: Record and categorize expenses with date tracking
- **Budget Planning**: Set category-based budgets with date ranges
- **Savings Goals**: Track progress towards financial goals
- **Category Analysis**: View expense breakdowns by category
- **Multi-currency Support**: Handle different currencies for wallets and expenses
- **RESTful API**: Clean, well-documented REST endpoints
- **Type-safe Database**: SQLx for compile-time SQL verification
- **Async Runtime**: High-performance async operations with Tokio
- **Docker Support**: Containerized deployment with Docker Compose
- **Comprehensive Testing**: Unit tests, integration tests, and Postman collection with automated test scripts

## Tech Stack

- **Language**: Rust 1.85+ (Edition 2024)
- **Web Framework**: Actix-web 4.9
- **Database**: PostgreSQL 15+ with SQLx 0.8
- **Async Runtime**: Tokio 1.48
- **Authentication**: JWT (jsonwebtoken 9.3) + bcrypt 0.16
- **Serialization**: Serde 1.0
- **Logging**: Tracing + tracing-subscriber
- **Testing**: Tokio-test, Mockall
- **Containerization**: Docker & Docker Compose

## Project Structure

```
finGest-rs/
├── migrations/                    # SQL database migrations
│   ├── 20230610000000_initial_schema.sql
│   └── 20250611000000_sample_data.sql
├── src/
│   ├── main.rs                   # Application entry point
│   ├── lib.rs                    # Library exports
│   ├── api/                      # API layer
│   │   ├── handlers/             # Request handlers
│   │   │   ├── auth_handler.rs   # Authentication endpoints
│   │   │   ├── category_handler.rs
│   │   │   └── user_handler.rs   # User, wallet, expense, budget handlers
│   │   ├── routes/               # Route configuration
│   │   │   ├── auth_routes.rs
│   │   │   ├── category_routes.rs
│   │   │   └── user_routes.rs
│   │   └── middleware.rs         # JWT authentication middleware
│   ├── config/                   # Configuration management
│   │   └── mod.rs                # Environment-based config
│   ├── db/                       # Database layer
│   │   ├── schema.rs             # Database schema types
│   │   └── migrations.rs         # Migration runner
│   ├── models/                   # Domain models
│   │   ├── user.rs               # User and account models
│   │   ├── wallet.rs             # Wallet management
│   │   ├── expense.rs            # Expense tracking
│   │   ├── budget.rs             # Budget planning
│   │   ├── category.rs           # Expense categories
│   │   ├── saving.rs             # Savings goals
│   │   ├── money.rs              # Money type (amount + currency)
│   │   └── summary.rs            # Financial summaries
│   ├── services/                 # Business logic layer
│   │   ├── auth_service.rs       # Authentication service
│   │   ├── user_service.rs       # User operations
│   │   ├── category_service.rs   # Category operations
│   │   └── mocks.rs              # Mock implementations for testing
│   ├── errors/                   # Error handling
│   │   └── mod.rs                # Custom error types
│   └── utils/                    # Utilities
│       └── validation.rs         # Input validation
├── tests/                        # Integration tests
│   └── api_tests.rs
├── docs/                         # Documentation
│   ├── authentication.md         # Auth implementation details
│   ├── api_examples.md           # API usage examples
│   ├── quick_start.md            # Quick start guide
│   └── money_manager_architecture.md
├── scripts/                      # Helper scripts
│   ├── setup_db.sh               # Database setup
│   ├── prepare_sqlx.sh           # SQLx offline mode prep
│   └── build_and_run.sh          # Build and run script
├── Cargo.toml                    # Rust dependencies
├── Dockerfile                    # Container image definition
├── docker-compose.yml            # Multi-container setup
├── sample.env                    # Environment variable template
└── README.md                     # This file
```

## Getting Started

### Prerequisites

- **Rust**: 1.85.0 or later ([Install Rust](https://rustup.rs/))
- **PostgreSQL**: 15 or later
- **Docker** (optional): For containerized deployment
- **Docker Compose** (optional): For orchestrating services

### Environment Setup

1. **Clone the repository**:

```bash
git clone https://github.com/Borelli-7/finGest-rs.git
cd finGest-rs
```

2. **Create environment configuration**:

```bash
cp sample.env .env
```

3. **Configure environment variables** in `.env`:

```env
# Server Configuration
HOST=0.0.0.0
PORT=8080

# Database Configuration
DATABASE_URL=postgres://postgres:password@localhost:5432/moneymanager
DB_MAX_CONNECTIONS=10

# Logging
RUST_LOG=info

# JWT Authentication
# IMPORTANT: Generate a secure secret for production!
# Use: openssl rand -base64 32
JWT_SECRET=your-secret-key-min-32-characters-change-in-production
JWT_EXPIRATION_HOURS=24
```

**Security Note**: Always generate a secure JWT secret for production:
```bash
openssl rand -base64 32
```

### Running the Application

#### Option 1: Using Cargo (Local Development)

1. **Start PostgreSQL** (if not using Docker):

```bash
docker compose up db -d
```

2. **Run the application**:

```bash
cargo run
```

The server will start at `http://localhost:8080`.

#### Option 2: Using Docker Compose (Recommended)

```bash
docker compose up
```

This starts both the API server and PostgreSQL database.

#### Option 3: Using Build Scripts

```bash
./scripts/build_and_run.sh
```

### Database Migrations

Database migrations run automatically on application startup. Migration files are located in the `migrations/` directory:

- `20230610000000_initial_schema.sql` - Initial database schema
- `20250611000000_sample_data.sql` - Sample data for testing

To manually set up the database:

```bash
./scripts/setup_db.sh [username] [password] [database_name]
```

## Authentication

FinGest uses JWT (JSON Web Tokens) for secure authentication. The implementation follows OWASP security best practices.

### Security Features

- **bcrypt** password hashing (cost factor: 12)
- **JWT tokens** with HS256 algorithm
- **Configurable token expiration** (default: 24 hours)
- **Protected routes** via middleware
- **Input validation** on all endpoints

### Authentication Endpoints

#### Register a New User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "login": "john_doe",
    "firstName": "John",
    "lastName": "Doe",
    "password": "SecurePassword123!",
    "admin": false
  }'
```

**Response** (201 Created):
```json
{
  "login": "john_doe",
  "firstName": "John",
  "lastName": "Doe",
  "admin": false
}
```

#### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "john_doe",
    "password": "SecurePassword123!"
  }'
```

**Response** (200 OK):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "login": "john_doe",
    "firstName": "John",
    "lastName": "Doe",
    "admin": false
  }
}
```

#### Verify Token

```bash
curl -X GET http://localhost:8080/api/auth/verify \
  -H "Authorization: Bearer <your-token>"
```

**Response** (200 OK):
```json
{
  "valid": true,
  "login": "john_doe",
  "admin": false
}
```

For detailed authentication documentation, see [docs/authentication.md](docs/authentication.md).

## API Documentation

### Base URL

```
http://localhost:8080
```

### Endpoints Overview

#### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login and receive JWT token
- `GET /api/auth/verify` - Verify JWT token validity

#### Categories
- `GET /resources/categories` - Get all available expense categories

#### Users
- `GET /resources/users` - Get all users
- `PUT /resources/users/{login}?field=fieldName` - Update user field

#### Wallets
- `GET /resources/users/{login}/wallets` - Get user's wallets
- `POST /resources/users/{login}/wallets` - Create a new wallet
- `GET /resources/users/{login}/wallets/{id}/summary` - Get wallet summary

#### Expenses
- `GET /resources/users/{login}/wallets/{id}/expenses` - Get wallet expenses
- `POST /resources/users/{login}/wallets/{id}/expenses` - Add expense to wallet
- `DELETE /resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}` - Delete an expense
- `GET /resources/users/{login}/wallets/{id}/highest_expense` - Get highest expense
- `GET /resources/users/{login}/wallets/{id}/counted_categories` - Get expense breakdown by category

#### Budgets
- `GET /resources/users/{login}/budgets` - Get user's budgets
- `POST /resources/users/{login}/budgets` - Create a new budget

### Example API Calls

#### Create a Wallet

```bash
curl -X POST http://localhost:8080/resources/users/john_doe/wallets \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{
    "name": "My Main Wallet",
    "amount": {
      "amount": "1000.00",
      "currency": "USD"
    }
  }'
```

#### Add an Expense

```bash
curl -X POST http://localhost:8080/resources/users/john_doe/wallets/1/expenses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{
    "amount": {
      "amount": "50.00",
      "currency": "USD"
    },
    "date": "2025-11-29",
    "description": "Grocery shopping",
    "category": {
      "name": "Food",
      "profit": false
    }
  }'
```

#### Create a Budget

```bash
curl -X POST http://localhost:8080/resources/users/john_doe/budgets \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{
    "category": {
      "name": "Food",
      "profit": false
    },
    "total": {
      "amount": "500.00",
      "currency": "USD"
    },
    "dateRange": {
      "start": "2025-11-01",
      "end": "2025-11-30"
    }
  }'
```

For more API examples, see [docs/api_examples.md](docs/api_examples.md).

## Database Schema

The application uses PostgreSQL with the following main tables:

- **account** - User accounts with authentication credentials
- **category** - Expense/income categories
- **wallet** - User wallets with currency support
- **account_wallet** - Many-to-many relationship between accounts and wallets
- **expense** - Expense records linked to wallets and categories
- **budget** - Budget allocations by category and date range
- **saving** - Savings goals with progress tracking

### Entity Relationships

```
account (1) ---> (N) account_wallet (N) ---> (1) wallet
   |                                             |
   |                                             |
  (1)                                           (1)
   |                                             |
   v                                             v
budget (N) <--- category ---> (N) expense (N) ---+
   |                              |
  (N)                            (1)
```

## Development

### Building the Project

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### SQLx Offline Mode

To prepare SQLx for offline compilation:

```bash
./scripts/prepare_sqlx.sh
```

This generates `sqlx-data.json` for compile-time query verification without a database connection.

## Testing

### Unit and Integration Tests

Integration tests are located in `tests/api_tests.rs`. They test the full API stack including database interactions.

Run integration tests:

```bash
# Run all tests
cargo test

# Run integration tests only
cargo test --test api_tests

# Run tests with output
cargo test -- --nocapture
```

### Postman Collection Testing

The project includes comprehensive Postman collection and environment files for interactive API testing. These files provide ready-to-use requests for all endpoints with automated test scripts.

#### What's Included

- **Complete API Coverage**: All authentication, user, wallet, expense, budget, and category endpoints
- **Automated Test Scripts**: Each request includes validation for status codes, response structure, and business logic
- **Error Case Testing**: Comprehensive error scenario validation (400, 401, 404 responses)
- **Environment Management**: Pre-configured local environment with dynamic variables
- **Workflow Automation**: Auto-populating IDs and tokens across requests

#### Files Location

```
tests/
├── postman_collection.json          # Complete Postman collection
├── postman_environment_local.json   # Local environment configuration
└── POSTMAN_TESTING_GUIDE.md        # Detailed testing guide
```

#### Quick Start with Postman

1. **Import the Collection**:
   - Open Postman
   - Click **Import**
   - Select `tests/postman_collection.json` and `tests/postman_environment_local.json`
   - Click **Import**

2. **Select the Environment**:
   - Click the environment dropdown (top-right)
   - Select **FinGest Local Environment**

3. **Start the Server**:
   ```bash
   docker compose up -d
   cargo run --release
   ```

4. **Run Tests**:
   - Right-click on **FinGest Money Manager API** collection
   - Select **Run collection**
   - Click **Run** to execute all tests

#### Collection Structure

The Postman collection is organized into folders:

- **Authentication** (7 requests)
  - User registration (success + error cases)
  - Login (success + error cases)
  - Token verification (success + error cases)

- **Users** (3 requests)
  - Get all users
  - Update user (success + error cases)

- **Categories** (1 request)
  - Get all categories

- **Wallets** (4 requests)
  - Get user wallets
  - Create wallet (success + error cases)
  - Get wallet summary

- **Expenses** (8 requests)
  - List, create, delete expenses
  - Get highest expense
  - Get category breakdown
  - Error case validations

- **Budgets** (4 requests)
  - List and create budgets
  - Date range validation
  - Required field validation

#### Automated Test Validation

Each request includes test scripts that automatically verify:

✅ **Success Cases**:
- Correct HTTP status codes (200, 201, 204)
- Response body structure and required fields
- Data type validation
- Response time assertions
- Automatic token and ID extraction

❌ **Error Cases**:
- Proper error status codes (400, 401, 404)
- Error message presence
- Validation failures
- Authentication errors

#### Environment Variables

The local environment includes:

- `base_url`: API base URL (http://localhost:8080)
- `auth_token`: JWT token (auto-populated after login)
- `test_user_*`: Test user credentials
- `test_wallet_id`, `test_expense_id`, `created_budget_id`: Auto-populated IDs
- `default_currency`: Default currency (PLN)
- `date_start_range`, `date_end_range`: Default date ranges

#### Testing Workflow

1. **First-Time Setup**:
   ```
   Register User → Login → Verify Token
   ```

2. **Normal Testing**:
   ```
   Login → Create Wallet → Add Expenses → Create Budget → Query Data
   ```

3. **Error Testing**:
   - Run error case requests to validate proper error handling
   - Test authentication failures
   - Verify validation rules

For detailed instructions, troubleshooting, and advanced usage, see [tests/POSTMAN_TESTING_GUIDE.md](tests/POSTMAN_TESTING_GUIDE.md).

### Apache JMeter Performance Testing

The project includes a comprehensive Apache JMeter test plan for performance, load, and stress testing of all API endpoints.

#### What's Included

- **Complete API Coverage**: All 16 endpoints with realistic test scenarios
- **Performance Benchmarking**: Response time and throughput measurement
- **Load Testing**: Simulates 50 concurrent users over 5 minutes
- **Stress Testing**: Tests system limits with 100 concurrent users
- **Error Case Validation**: Tests proper error handling (400, 401, 404)
- **Automated Assertions**: Validates response codes, JSON structure, and data
- **Realistic Test Data**: Dynamic user creation, random amounts, timestamps
- **Comprehensive Reporting**: Summary, aggregate, and graphical results

#### Files Location

```
tests/
├── fingest_performance_test.jmx    # JMeter test plan
├── JMETER_TESTING_GUIDE.md         # Detailed testing guide
└── API_ENDPOINT_COVERAGE.md        # Complete endpoint documentation
```

#### Quick Start with JMeter

1. **Install Apache JMeter** (requires Java 8+):
   ```bash
   wget https://archive.apache.org/dist/jmeter/binaries/apache-jmeter-5.6.3.tgz
   tar -xzf apache-jmeter-5.6.3.tgz
   ```

2. **Start the API**:
   ```bash
   docker compose up -d
   cargo run --release
   ```

3. **Run Performance Tests**:
   ```bash
   # GUI mode (for test development)
   apache-jmeter-5.6.3/bin/jmeter -t tests/fingest_performance_test.jmx
   
   # CLI mode (for actual performance testing)
   apache-jmeter-5.6.3/bin/jmeter -n \
     -t tests/fingest_performance_test.jmx \
     -l results.jtl \
     -e -o report/
   ```

4. **View Results**:
   ```bash
   # Open HTML report
   open report/index.html
   ```

#### Test Plan Structure

The JMeter test plan includes **7 thread groups**:

1. **Authentication Flow** (10 threads) - Registration, login, token verification
2. **Categories and Users** (5 threads) - Category and user operations
3. **Wallet Operations** (10 threads) - Wallet CRUD with validation
4. **Expense Operations** (15 threads) - Expense tracking and analytics
5. **Budget Operations** (8 threads) - Budget management
6. **Load Test** (50 threads, disabled by default) - Sustained load testing
7. **Stress Test** (100 threads, disabled by default) - High-volume testing

#### Key Features

- ✅ **16/16 endpoints tested** with 24 test scenarios
- ✅ **JWT authentication** with automatic token extraction
- ✅ **Dynamic test data** generation for unique test runs
- ✅ **Response assertions** for all requests
- ✅ **Error case testing** for validation and edge cases
- ✅ **Configurable variables** for different environments
- ✅ **Multiple listeners** for different analysis needs
- ✅ **CI/CD ready** for automated performance testing

#### Performance Targets

- Simple GET requests: < 100ms
- POST/PUT requests: < 200ms
- Complex queries: < 500ms
- Error rate: 0% for functional tests
- Throughput: Should scale linearly with thread count

For detailed instructions, configuration options, and troubleshooting, see [tests/JMETER_TESTING_GUIDE.md](tests/JMETER_TESTING_GUIDE.md).

## Docker Deployment

### Building the Docker Image

```bash
docker build -t fingest-rs:latest .
```

### Running with Docker Compose

```bash
# Start all services
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down

# Stop and remove volumes
docker compose down -v
```

### Docker Configuration

The `docker-compose.yml` defines:
- **api**: The Rust API server (port 8080)
- **db**: PostgreSQL database (port 5432)
- **networks**: Bridge network for service communication
- **volumes**: Persistent storage for PostgreSQL data

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Format code (`cargo fmt`)
6. Run linter (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Additional Resources:**
- [Quick Start Guide](docs/quick_start.md)
- [Authentication Documentation](docs/authentication.md)
- [API Examples](docs/api_examples.md)
- [Postman Testing Guide](tests/POSTMAN_TESTING_GUIDE.md)
- [JMeter Testing Guide](tests/JMETER_TESTING_GUIDE.md)
- [API Endpoint Coverage](tests/API_ENDPOINT_COVERAGE.md)
- [Architecture Overview](docs/money_manager_architecture.md)

**Questions or Issues?** Please open an issue on GitHub.
