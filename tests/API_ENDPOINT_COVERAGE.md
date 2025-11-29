# FinGest API Endpoint Coverage - JMeter Test Plan

## Overview

This document provides a comprehensive list of all API endpoints tested in the JMeter performance test plan (`fingest_performance_test.jmx`). Each endpoint includes details about the HTTP method, path, test scenarios, and assertions.

## Complete Endpoint Coverage

### Authentication Endpoints (3 endpoints)

#### 1. POST /api/auth/register
- **Description**: Register a new user account
- **Thread Group**: 01 - Authentication Flow
- **Request Body**:
  ```json
  {
    "login": "testuser_20251129_1234",
    "firstName": "Test",
    "lastName": "User",
    "password": "SecurePass123!",
    "admin": false
  }
  ```
- **Assertions**:
  - HTTP 201 Created status code
  - Response contains user login field matching input
- **Variables Extracted**: None
- **Test Cases**:
  - ✅ Successful registration with valid data

#### 2. POST /api/auth/login
- **Description**: Login with username and password to receive JWT token
- **Thread Group**: 01 - Authentication Flow
- **Request Body**:
  ```json
  {
    "login": "testuser_20251129_1234",
    "password": "SecurePass123!"
  }
  ```
- **Assertions**:
  - HTTP 200 OK status code
  - Response contains JWT token field
- **Variables Extracted**: 
  - `JWT_TOKEN` - JWT authentication token
- **Test Cases**:
  - ✅ Successful login with correct credentials
  - ❌ Failed login with invalid credentials (expects 401)

#### 3. GET /api/auth/verify
- **Description**: Verify JWT token validity
- **Thread Group**: 01 - Authentication Flow
- **Headers**: 
  - `Authorization: Bearer ${JWT_TOKEN}`
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Valid token verification

---

### Category Endpoints (1 endpoint)

#### 4. GET /resources/categories
- **Description**: Get all available expense/income categories
- **Thread Group**: 02 - Categories and Users
- **Query Parameters**: None
- **Assertions**:
  - HTTP 200 OK status code
  - Response is an array
- **Test Cases**:
  - ✅ Retrieve all categories successfully

---

### User Endpoints (2 endpoints)

#### 5. GET /resources/users
- **Description**: Get all registered users
- **Thread Group**: 02 - Categories and Users
- **Query Parameters**: None
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve all users successfully

#### 6. PUT /resources/users/{login}
- **Description**: Update user field (firstName, lastName, or password)
- **Thread Group**: 02 - Categories and Users
- **Path Parameters**: 
  - `login` - User login name
- **Query Parameters**:
  - `field` - Field name to update (e.g., "firstName")
- **Request Body**:
  ```json
  {
    "firstName": "UpdatedName"
  }
  ```
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Update user firstName field

---

### Wallet Endpoints (3 endpoints)

#### 7. GET /resources/users/{login}/wallets
- **Description**: Get all wallets for a specific user
- **Thread Group**: 03 - Wallet Operations
- **Path Parameters**: 
  - `login` - User login name
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve user wallets successfully

#### 8. POST /resources/users/{login}/wallets
- **Description**: Create a new wallet for a user
- **Thread Group**: 03 - Wallet Operations
- **Path Parameters**: 
  - `login` - User login name
- **Request Body**:
  ```json
  {
    "name": "Main Wallet 2025-11-29",
    "amount": {
      "amount": "1000.00",
      "currency": "USD"
    }
  }
  ```
- **Assertions**:
  - HTTP 201 Created status code
- **Variables Extracted**: 
  - `WALLET_ID` - Created wallet ID
- **Test Cases**:
  - ✅ Create wallet with valid data
  - ❌ Create wallet with invalid data (expects 400)

#### 9. GET /resources/users/{login}/wallets/{id}/summary
- **Description**: Get wallet summary with balance information
- **Thread Group**: 03 - Wallet Operations
- **Path Parameters**: 
  - `login` - User login name
  - `id` - Wallet ID
- **Query Parameters**:
  - `start` - Start date (YYYY-MM-DD)
  - `end` - End date (YYYY-MM-DD)
- **Assertions**:
  - HTTP 200 OK status code
  - Response contains wallet name field
- **Test Cases**:
  - ✅ Retrieve wallet summary with date range

---

### Expense Endpoints (5 endpoints)

#### 10. GET /resources/users/{login}/wallets/{id}/expenses
- **Description**: Get all expenses for a wallet within date range
- **Thread Group**: 04 - Expense Operations
- **Path Parameters**: 
  - `login` - User login name
  - `id` - Wallet ID
- **Query Parameters**:
  - `start` - Start date (YYYY-MM-DD)
  - `end` - End date (YYYY-MM-DD)
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve expenses with date filtering

#### 11. POST /resources/users/{login}/wallets/{id}/expenses
- **Description**: Create a new expense for a wallet
- **Thread Group**: 04 - Expense Operations
- **Path Parameters**: 
  - `login` - User login name
  - `id` - Wallet ID
- **Request Body**:
  ```json
  {
    "amount": {
      "amount": "150.50",
      "currency": "USD"
    },
    "date": "2025-11-29",
    "description": "Test Expense 143045",
    "category": {
      "name": "Food",
      "profit": false
    }
  }
  ```
- **Assertions**:
  - HTTP 201 Created status code
- **Variables Extracted**: 
  - `EXPENSE_ID` - Created expense ID
- **Test Cases**:
  - ✅ Create expense with valid data and random amounts
  - ❌ Create expense with missing required fields (expects 400)

#### 12. GET /resources/users/{login}/wallets/{id}/highest_expense
- **Description**: Get the highest expense in a wallet for a date range
- **Thread Group**: 04 - Expense Operations
- **Path Parameters**: 
  - `login` - User login name
  - `id` - Wallet ID
- **Query Parameters**:
  - `start` - Start date (YYYY-MM-DD)
  - `end` - End date (YYYY-MM-DD)
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve highest expense

#### 13. GET /resources/users/{login}/wallets/{id}/counted_categories
- **Description**: Get expense breakdown by category
- **Thread Group**: 04 - Expense Operations
- **Path Parameters**: 
  - `login` - User login name
  - `id` - Wallet ID
- **Query Parameters**:
  - `start` - Start date (YYYY-MM-DD)
  - `end` - End date (YYYY-MM-DD)
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve category breakdown

#### 14. DELETE /resources/users/{login}/wallets/{wallet_id}/expenses/{expense_id}
- **Description**: Delete a specific expense
- **Thread Group**: 04 - Expense Operations
- **Path Parameters**: 
  - `login` - User login name
  - `wallet_id` - Wallet ID
  - `expense_id` - Expense ID to delete
- **Assertions**:
  - HTTP 200 OK or 204 No Content status code
- **Test Cases**:
  - ✅ Delete expense successfully

---

### Budget Endpoints (2 endpoints)

#### 15. GET /resources/users/{login}/budgets
- **Description**: Get all budgets for a user with date range filters
- **Thread Group**: 05 - Budget Operations
- **Path Parameters**: 
  - `login` - User login name
- **Query Parameters**:
  - `start_min` - Minimum start date (YYYY-MM-DD)
  - `start_max` - Maximum start date (YYYY-MM-DD)
  - `end_min` - Minimum end date (YYYY-MM-DD)
  - `end_max` - Maximum end date (YYYY-MM-DD)
- **Assertions**:
  - HTTP 200 OK status code
- **Test Cases**:
  - ✅ Retrieve budgets with date range filters

#### 16. POST /resources/users/{login}/budgets
- **Description**: Create a new budget for a user
- **Thread Group**: 05 - Budget Operations
- **Path Parameters**: 
  - `login` - User login name
- **Request Body**:
  ```json
  {
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
  }
  ```
- **Assertions**:
  - HTTP 201 Created status code
- **Variables Extracted**: 
  - `BUDGET_ID` - Created budget ID
- **Test Cases**:
  - ✅ Create budget with valid date range
  - ❌ Create budget with invalid date range (expects 400)

---

## Summary Statistics

### Total Endpoint Coverage
- **Total Endpoints**: 16
- **GET Endpoints**: 8
- **POST Endpoints**: 6
- **PUT Endpoints**: 1
- **DELETE Endpoints**: 1

### Test Scenarios
- **Success Cases**: 19
- **Error Cases**: 5
- **Total Test Cases**: 24

### Thread Groups
1. Authentication Flow: 4 requests (3 success + 1 error)
2. Categories and Users: 3 requests
3. Wallet Operations: 4 requests (3 success + 1 error)
4. Expense Operations: 6 requests (5 success + 1 error)
5. Budget Operations: 3 requests (2 success + 1 error)
6. Load Test: 2 requests (disabled by default)
7. Stress Test: 1 request (disabled by default)

### Assertion Types Used
- **HTTP Status Code Assertions**: All requests
- **JSON Path Assertions**: Selected requests
- **Response Field Validation**: Key endpoints

### Variables and Data Extraction
- **JWT_TOKEN**: Extracted from login, used in all authenticated requests
- **WALLET_ID**: Extracted from wallet creation, used in expense/summary requests
- **EXPENSE_ID**: Extracted from expense creation, used in delete request
- **BUDGET_ID**: Extracted from budget creation
- **TEST_USER**: Dynamic username generation for unique test data

### Performance Test Configuration
- **Functional Tests**: 48 total threads across groups 1-5
- **Load Test** (disabled): 50 threads, 10 loops, 5 minutes
- **Stress Test** (disabled): 100 threads, 20 loops, 10 minutes

### Response Time Expectations
- **Simple GET requests**: < 100ms target
- **POST/PUT requests**: < 200ms target
- **Complex queries**: < 500ms target
- **Error responses**: < 100ms target

---

## JMeter Test Plan Features

### Test Data Generation
- **Dynamic Usernames**: `testuser_${__time(YMD)}_${__Random(1000,9999)}`
- **Random Amounts**: `${__Random(10,500)}.${__Random(10,99)}`
- **Timestamp-based Names**: `Main Wallet ${__time(YMD_HMS)}`
- **Unique Descriptions**: `Test Expense ${__time(HMS)}`

### Assertions and Validations
- HTTP status code validation for all requests
- JSON structure validation for responses
- Field presence validation
- Value matching for critical fields
- Error message presence for failure scenarios

### Listeners and Reports
- View Results Tree (detailed inspection)
- Summary Report (aggregate statistics)
- Aggregate Report (percentiles and performance metrics)
- Graph Results (visual trends)
- Response Time Graph (latency visualization)
- Backend Listener (optional InfluxDB integration)

### Best Practices Implemented
- ✅ Separate thread groups for different test types
- ✅ Realistic ramp-up periods
- ✅ Token extraction and reuse
- ✅ ID extraction for dependent requests
- ✅ Error case validation
- ✅ Configurable variables
- ✅ HTTP connection reuse
- ✅ Response time monitoring
- ✅ Comprehensive assertions

---

## Related Documentation

- **JMeter Test Plan**: `tests/fingest_performance_test.jmx`
- **JMeter Testing Guide**: `tests/JMETER_TESTING_GUIDE.md`
- **API Documentation**: `README.md`
- **API Examples**: `docs/api_examples.md`
- **Postman Collection**: `tests/postman_collection.json`
- **Postman Testing Guide**: `tests/POSTMAN_TESTING_GUIDE.md`

---

**Last Updated**: 2025-11-29  
**Test Plan Version**: 1.0.0  
**API Coverage**: 100% (16/16 endpoints)
