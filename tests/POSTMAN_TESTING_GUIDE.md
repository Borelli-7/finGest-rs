# Postman Collection Testing Guide

## Overview

This directory contains comprehensive Postman collection and environment files for testing the FinGest Money Manager API. The collection includes all available endpoints with success cases, error cases, and automated test scripts.

## Files

- **`postman_collection.json`** - Complete Postman collection with all API endpoints
- **`postman_environment_local.json`** - Local environment configuration with variables

## Quick Start

### 1. Import into Postman

1. Open Postman
2. Click **Import** button
3. Select both files:
   - `postman_collection.json`
   - `postman_environment_local.json`
4. Click **Import**

### 2. Select Environment

1. In Postman, click the environment dropdown (top-right)
2. Select **FinGest Local Environment**

### 3. Start Your Local Server

```bash
# Make sure your database is running
docker-compose up -d

# Run the API server
cargo run --release
```

The server should be running at `http://localhost:8080`

### 4. Run the Collection

**Option A: Run All Tests**
1. Right-click on the collection **FinGest Money Manager API**
2. Select **Run collection**
3. Click **Run FinGest Money Manager API**

**Option B: Run Individual Folders**
1. Expand the collection
2. Right-click on any folder (e.g., "Authentication", "Wallets")
3. Select **Run folder**

**Option C: Run Individual Requests**
1. Click on any request
2. Click the **Send** button

## Collection Structure

### 1. Authentication Folder
- **Register User - Success**: Create a new user account
- **Register User - Duplicate Login (Error)**: Test duplicate username validation
- **Register User - Invalid Password (Error)**: Test password length validation
- **Login - Success**: Authenticate and receive JWT token
- **Login - Invalid Credentials (Error)**: Test authentication failure
- **Verify Token - Success**: Validate JWT token
- **Verify Token - Missing Token (Error)**: Test unauthorized access

### 2. Users Folder
- **Get All Users - Success**: Retrieve all users
- **Update User - Success**: Update user information
- **Update User - Non-existent User (Error)**: Test 404 handling

### 3. Categories Folder
- **Get All Categories - Success**: Retrieve all available categories

### 4. Wallets Folder
- **Get User Wallets - Success**: Get all wallets for a user
- **Create Wallet - Success**: Create a new wallet
- **Create Wallet - Invalid Amount (Error)**: Test validation
- **Get Wallet Summary - Success**: Get wallet statistics

### 5. Expenses Folder
- **Get Wallet Expenses - Success**: Retrieve all expenses
- **Create Expense - Success**: Add a new expense
- **Create Expense - Invalid Date (Error)**: Test date validation
- **Delete Expense - Success**: Remove an expense
- **Delete Expense - Non-existent (Error)**: Test 404 handling
- **Get Highest Expense - Success**: Find the largest expense
- **Get Counted Categories - Success**: Get expense counts by category
- **Get Counted Categories - Invalid Date Range (Error)**: Test query parameter validation

### 6. Budgets Folder
- **Get User Budgets - Success**: Retrieve user budgets
- **Create Budget - Success**: Create a new budget
- **Create Budget - Invalid Date Range (Error)**: Test date range validation
- **Create Budget - Missing Fields (Error)**: Test required field validation

## Environment Variables

The local environment includes the following variables:

### Server Configuration
- `base_url`: API base URL (default: `http://localhost:8080`)

### Authentication
- `auth_token`: JWT token (auto-populated after login)
- `test_user_login`: Test user username
- `test_user_password`: Test user password
- `test_user_firstname`: Test user first name
- `test_user_lastname`: Test user last name

### Test Data IDs
- `test_wallet_id`: Wallet ID for testing (auto-populated)
- `created_wallet_id`: Newly created wallet ID
- `test_expense_id`: Expense ID for testing (auto-populated)
- `created_expense_id`: Newly created expense ID
- `created_budget_id`: Newly created budget ID

### Default Values
- `default_currency`: Default currency code (PLN)
- `test_category_food`: Food category name
- `test_category_transport`: Transport category name
- `test_category_entertainment`: Entertainment category name
- `date_start_range`: Default start date for queries
- `date_end_range`: Default end date for queries
- `max_response_time_ms`: Maximum acceptable response time

## Test Scripts

Each request includes comprehensive test scripts that validate:

### Success Cases
- ✅ Correct HTTP status codes (200, 201, etc.)
- ✅ Response body structure and required fields
- ✅ Data type validation
- ✅ Response time assertions
- ✅ Automatic variable extraction (IDs, tokens)

### Error Cases
- ❌ Proper error status codes (400, 401, 404)
- ❌ Error message presence
- ❌ Validation error handling
- ❌ Authentication failures
- ❌ Not found scenarios

## Recommended Testing Workflow

### First-Time Setup
1. **Register User** - Create a test account
2. **Login** - Authenticate and get token
3. **Verify Token** - Confirm authentication works

### Normal Workflow
1. Run **Login** to get a fresh token
2. Create resources (Wallet, Expenses, Budgets)
3. Test GET endpoints
4. Test UPDATE/DELETE operations
5. Run error cases to validate error handling

### Automated Testing
Run the entire collection to execute all tests in sequence. The collection is designed to:
- Auto-populate IDs from creation responses
- Chain authentication across requests
- Validate both success and failure scenarios

## Customization

### Changing the Base URL
1. Go to **Environments** → **FinGest Local Environment**
2. Edit `base_url` value
3. Save the environment

### Using Different Test Data
1. Edit environment variables for user credentials
2. Modify request bodies in individual requests
3. Update test scripts if needed

### Adding New Tests
1. Duplicate an existing request
2. Modify the URL, body, and parameters
3. Update test scripts to match expected behavior

## Troubleshooting

### "Could not get any response"
- Verify the server is running on `http://localhost:8080`
- Check if the database is accessible
- Confirm no firewall is blocking the connection

### Authentication Errors
- Run the **Login** request first to get a valid token
- Check that `auth_token` is set in the environment
- Verify the test user exists in the database

### Failed Tests
- Check the **Test Results** tab in Postman
- Review the response body for error messages
- Ensure the database has required sample data

### Variable Not Set
- Some variables are auto-populated from responses
- Run prerequisite requests first (e.g., create before delete)
- Check the test scripts in the **Pre-request Script** and **Tests** tabs

## API Documentation Reference

For detailed API documentation, see:
- `docs/api_examples.md` - cURL examples for all endpoints
- `docs/authentication.md` - Authentication flow and security details
- `docs/quick_start.md` - Getting started guide

## Testing Best Practices

1. **Run in Order**: For first-time testing, run requests in the order they appear
2. **Clean State**: Use a fresh database for consistent test results
3. **Environment Isolation**: Use separate environments for dev/staging/production
4. **Review Logs**: Check server logs for detailed error information
5. **Update Variables**: Keep test data variables updated with valid values

## Advanced Features

### Pre-request Scripts
Some requests include pre-request scripts that:
- Generate dynamic data (timestamps, random values)
- Set up test prerequisites
- Calculate derived values

### Collection Variables
The collection includes variables that can be referenced across all requests:
- Use `{{variable_name}}` syntax in URLs, headers, and bodies
- Variables are automatically substituted at runtime

### Response Validation
Test scripts validate:
- HTTP status codes
- Response structure
- Data types
- Business logic rules
- Performance metrics

## Contributing

When adding new endpoints to the API:
1. Add corresponding requests to the collection
2. Include both success and error cases
3. Write comprehensive test scripts
4. Update environment variables if needed
5. Document the changes in this README

## Support

For issues or questions:
- Review the test output in Postman's console
- Check server logs for backend errors
- Refer to the API documentation in the `docs/` folder
- Verify database state matches test expectations
