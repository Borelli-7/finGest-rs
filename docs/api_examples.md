# API Test Examples

This document provides examples for testing the FinGest API using curl. You can use these commands as a reference for interacting with the API.

## Category Endpoints

### Get all categories

```bash
curl -X GET http://localhost:8080/resources/categories
```

## User Endpoints

### Get all users

```bash
curl -X GET http://localhost:8080/resources/users
```

### Update user

```bash
curl -X PUT "http://localhost:8080/resources/users/user1?field=firstName" \
  -H "Content-Type: application/json" \
  -d '{"firstName": "John"}'
```

## Wallet Endpoints

### Get wallets for user

```bash
curl -X GET http://localhost:8080/resources/users/user1/wallets
```

### Create wallet

```bash
curl -X POST http://localhost:8080/resources/users/user2/wallets \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Wallet",
    "amount": {
      "amount": "1000.00",
      "currency": "PLN"
    }
  }'
```

### Get wallet summary

```bash
curl -X GET "http://localhost:8080/resources/users/user2/wallets"
```

### Get wallet expenses

```bash
curl -X GET "http://localhost:8080/resources/users/user1/wallets/4/expenses"
```

### Add expense

```bash
curl -X POST http://localhost:8080/resources/users/user1/wallets/3/expenses \
  -H "Content-Type: application/json" \
  -d '{
    "amount": {
      "amount": "50.00",
      "currency": "PLN"
    },
    "date": "2023-06-10",
    "description": "Grocery shopping",
    "category": {
      "name": "Food",
      "profit": false
    }
  }'
```

### Delete expense

```bash
curl -X DELETE http://localhost:8080/resources/users/user1/wallets/1/expenses/1
```

### Get highest expense

```bash
curl -X GET "http://localhost:8080/resources/users/user1/wallets/4/highest_expense"
```

### Get counted categories

```bash
curl -X GET "http://localhost:8080/resources/users/user1/wallets/4/counted_categories?start=2023-01-01&end=2025-12-31"
```

## Budget Endpoints

### Get budgets

```bash
curl -X GET "http://localhost:8080/resources/users/user2/budgets?start_min=2023-01-01&start_max=2023-12-31&end_min=2023-01-01&end_max=2023-12-31"
```

### Create budget

```bash
curl -X POST http://localhost:8080/resources/users/user2/budgets \
  -H "Content-Type: application/json" \
  -d '{
    "category": {
      "name": "Food",
      "profit": false
    },
    "total": {
      "amount": "500.00",
      "currency": "PLN"
    },
    "dateRange": {
      "start": "2023-06-01",
      "end": "2023-06-30"
    }
  }'
```
