# Apache JMeter Performance Testing Guide for FinGest API

## Overview

This guide provides comprehensive instructions for using the Apache JMeter test script (`fingest_performance_test.jmx`) to evaluate the performance, reliability, and edge cases of the FinGest Money Manager API.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Test Plan Overview](#test-plan-overview)
- [Configuration](#configuration)
- [Running the Tests](#running-the-tests)
- [Understanding the Results](#understanding-the-results)
- [Test Scenarios](#test-scenarios)
- [Customization](#customization)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Prerequisites

- **Apache JMeter**: Version 5.5 or later ([Download JMeter](https://jmeter.apache.org/download_jmeter.cgi))
- **Java**: JDK 8 or later (required for JMeter)
- **FinGest API**: Running locally or on a test server
- **PostgreSQL**: Database must be running and accessible

## Installation

### Installing Apache JMeter

1. **Download JMeter** from the [official website](https://jmeter.apache.org/download_jmeter.cgi)

2. **Extract the archive**:
   ```bash
   tar -xzf apache-jmeter-5.6.3.tgz
   cd apache-jmeter-5.6.3
   ```

3. **Verify installation**:
   ```bash
   bin/jmeter --version
   ```

4. **Add JMeter to PATH** (optional):
   ```bash
   export PATH=$PATH:/path/to/apache-jmeter-5.6.3/bin
   ```

### Starting the FinGest API

Before running tests, ensure the API is running:

```bash
# Using Docker Compose (recommended)
docker compose up -d

# Or using Cargo
cargo run --release
```

Verify the API is accessible:
```bash
curl http://localhost:8080/resources/categories
```

## Test Plan Overview

The JMeter test plan includes **7 thread groups** covering all API endpoints:

### Thread Groups

1. **01 - Authentication Flow** (10 threads, 5s ramp-up)
   - User registration
   - User login with JWT token extraction
   - Token verification
   - Invalid login scenarios

2. **02 - Categories and Users** (5 threads, 2s ramp-up)
   - Get all categories
   - Get all users
   - Update user fields

3. **03 - Wallet Operations** (10 threads, 5s ramp-up)
   - Get user wallets
   - Create new wallet
   - Get wallet summary
   - Error handling for invalid data

4. **04 - Expense Operations** (15 threads, 10s ramp-up)
   - Get expenses with date filtering
   - Create new expenses
   - Get highest expense
   - Get category breakdown
   - Delete expenses
   - Error handling

5. **05 - Budget Operations** (8 threads, 4s ramp-up)
   - Get user budgets
   - Create budgets with date ranges
   - Invalid date range validation

6. **06 - Load Test - Concurrent Operations** (50 threads, 30s ramp-up, DISABLED by default)
   - Simulates concurrent API access
   - Duration: 5 minutes (300 seconds)
   - 10 loops per thread

7. **07 - Stress Test - High Volume** (100 threads, 60s ramp-up, DISABLED by default)
   - High-volume expense creation
   - Duration: 10 minutes (600 seconds)
   - 20 loops per thread

### Listeners (Results Collectors)

- **View Results Tree**: Detailed request/response inspection
- **Summary Report**: Overall statistics
- **Aggregate Report**: Comprehensive performance metrics
- **Graph Results**: Visual performance trends
- **Response Time Graph**: Response time visualization
- **Backend Listener**: Optional real-time monitoring with InfluxDB (disabled by default)

## Configuration

### User Defined Variables

The test plan uses the following configurable variables:

| Variable | Default Value | Description |
|----------|---------------|-------------|
| `BASE_URL` | `localhost` | API server hostname |
| `PORT` | `8080` | API server port |
| `PROTOCOL` | `http` | Protocol (http/https) |
| `TEST_USER` | Dynamic | Auto-generated unique username |
| `TEST_PASSWORD` | `SecurePass123!` | Test user password |

### Modifying Variables

1. Open JMeter GUI:
   ```bash
   jmeter
   ```

2. Load the test plan: `File` → `Open` → Select `fingest_performance_test.jmx`

3. Navigate to: `Test Plan` → `User Defined Variables`

4. Modify values as needed

5. Save the test plan: `File` → `Save`

### Configuring for Remote Server

To test against a remote server:

```bash
# Edit variables in the test plan or use command-line parameters
jmeter -n -t fingest_performance_test.jmx \
  -JBASE_URL=api.example.com \
  -JPORT=443 \
  -JPROTOCOL=https \
  -l results.jtl
```

## Running the Tests

### GUI Mode (Development and Debugging)

**Note**: GUI mode is for test development only. Use CLI mode for actual performance testing.

```bash
# Open JMeter GUI
jmeter

# Load test plan
File → Open → tests/fingest_performance_test.jmx

# Run tests
Run → Start (or Ctrl+R)

# View results in real-time using listeners
```

### CLI Mode (Performance Testing)

**Recommended for actual performance testing:**

```bash
# Basic execution
jmeter -n -t tests/fingest_performance_test.jmx -l results/test_results.jtl

# With HTML report generation
jmeter -n -t tests/fingest_performance_test.jmx \
  -l results/test_results.jtl \
  -e -o results/html_report

# With custom log file
jmeter -n -t tests/fingest_performance_test.jmx \
  -l results/test_results.jtl \
  -j results/jmeter.log \
  -e -o results/html_report
```

### Running Specific Thread Groups

To run only specific thread groups, edit the test plan and enable/disable groups:

```bash
# In JMeter GUI:
# Right-click on thread group → Toggle (Enable/Disable)
```

Or programmatically:
```bash
# Disable thread groups 6 and 7 (load and stress tests) by default
# They are already disabled in the test plan
```

### Running Load and Stress Tests

The load test (Thread Group 6) and stress test (Thread Group 7) are disabled by default. To enable them:

1. Open the test plan in JMeter GUI
2. Right-click on the thread group
3. Select "Enable"
4. Save and run the test

**Warning**: Load and stress tests generate significant traffic. Only run against test environments.

## Understanding the Results

### Key Metrics

- **Samples**: Number of requests executed
- **Average**: Average response time in milliseconds
- **Min/Max**: Minimum and maximum response times
- **Std. Dev.**: Standard deviation of response times
- **Error %**: Percentage of failed requests
- **Throughput**: Requests per second
- **KB/sec**: Data transfer rate
- **Avg. Bytes**: Average response size

### Success Criteria

- **Error Rate**: Should be 0% for functional tests
- **Response Time**: 
  - Simple GET requests: < 100ms
  - POST/PUT requests: < 200ms
  - Complex queries: < 500ms
- **Throughput**: Should scale linearly with thread count

### Viewing Results

#### Summary Report

Shows aggregated statistics for all samplers:
```
Label                 | Samples | Average | Min | Max | Std.Dev | Error% | Throughput
POST /api/auth/login  | 10      | 45ms    | 35  | 75  | 12.3    | 0.00%  | 2.5/sec
```

#### Aggregate Report

More detailed statistics including percentiles:
```
Label                 | Median | 90% Line | 95% Line | 99% Line
POST /api/auth/login  | 42ms   | 65ms     | 70ms     | 75ms
```

#### HTML Report

Generated with the `-e` flag, provides:
- Dashboard with key metrics
- Response time charts
- Throughput graphs
- Error analysis
- Transaction summaries

Open the HTML report:
```bash
# Linux/Mac
open results/html_report/index.html

# Windows
start results/html_report/index.html
```

## Test Scenarios

### Functional Testing

The test plan includes comprehensive functional tests for all endpoints:

#### Authentication Endpoints
- ✅ User registration with valid data
- ✅ User login with correct credentials
- ✅ JWT token verification
- ❌ Login with invalid credentials (401)

#### Category Endpoints
- ✅ Get all categories
- ✅ Verify response structure

#### User Endpoints
- ✅ Get all users
- ✅ Update user fields
- ✅ Verify field updates

#### Wallet Endpoints
- ✅ Get user wallets
- ✅ Create wallet with valid data
- ✅ Get wallet summary
- ❌ Create wallet with invalid data (400)

#### Expense Endpoints
- ✅ Get expenses with date filtering
- ✅ Create expense with random amounts
- ✅ Get highest expense
- ✅ Get category breakdown
- ✅ Delete expense
- ❌ Create expense with missing fields (400)

#### Budget Endpoints
- ✅ Get user budgets
- ✅ Create budget with date range
- ❌ Create budget with invalid date range (400)

### Performance Testing

**Load Test (Thread Group 6)**:
- Simulates 50 concurrent users
- Tests sustained load over 5 minutes
- Verifies system stability under normal load

**Stress Test (Thread Group 7)**:
- Simulates 100 concurrent users
- Tests system limits over 10 minutes
- Identifies breaking points and bottlenecks

## Customization

### Adding New Test Cases

1. Open test plan in JMeter GUI
2. Right-click on thread group → Add → Sampler → HTTP Request
3. Configure the HTTP request:
   - Path: `/api/endpoint`
   - Method: GET/POST/PUT/DELETE
   - Body Data (for POST/PUT)
4. Add assertions:
   - Right-click on HTTP Request → Add → Assertions
   - Choose Response Assertion, JSON Assertion, etc.
5. Save the test plan

### Modifying Thread Configuration

To adjust load levels:

```xml
<!-- Edit in the .jmx file or in GUI -->
<stringProp name="ThreadGroup.num_threads">20</stringProp>  <!-- Number of users -->
<stringProp name="ThreadGroup.ramp_time">10</stringProp>    <!-- Ramp-up period in seconds -->
<stringProp name="LoopController.loops">5</stringProp>      <!-- Number of iterations -->
```

### Adding Custom Variables

1. Navigate to: Test Plan → User Defined Variables
2. Click "Add"
3. Set Name and Value
4. Use in requests with `${VARIABLE_NAME}`

### Parameterizing Test Data

Use CSV Data Set Config for data-driven testing:

1. Create CSV file: `test_data.csv`
   ```csv
   username,password,amount
   user1,pass1,100.00
   user2,pass2,200.00
   ```

2. Add CSV Data Set Config:
   - Right-click on Thread Group → Add → Config Element → CSV Data Set Config
   - Configure file path and variables

3. Use variables: `${username}`, `${password}`, `${amount}`

## Best Practices

### Performance Testing Best Practices

1. **Always run performance tests in CLI mode** (not GUI)
2. **Use a dedicated test environment** matching production
3. **Start with small load** and gradually increase
4. **Monitor server resources** (CPU, memory, database)
5. **Run tests multiple times** to ensure consistency
6. **Baseline performance** before making changes
7. **Document your results** for comparison

### JMeter Best Practices

1. **Use CSV files** for large datasets
2. **Extract dynamic values** (IDs, tokens) with extractors
3. **Add appropriate think time** between requests
4. **Use assertions** to validate responses
5. **Configure realistic ramp-up** periods
6. **Save results to files** for analysis
7. **Clean up test data** after tests

### API Testing Best Practices

1. **Test against test database** with sample data
2. **Use unique test users** (auto-generated usernames)
3. **Clean up resources** after tests
4. **Test both success and error cases**
5. **Validate edge cases** (empty data, large values)
6. **Test authentication** flows thoroughly
7. **Monitor database connections** and locks

## Troubleshooting

### Common Issues

#### Connection Refused
```
Error: Connection refused
```
**Solution**: Ensure the API is running:
```bash
curl http://localhost:8080/resources/categories
```

#### Port Already in Use
```
Error: Address already in use
```
**Solution**: Check if another process is using port 8080:
```bash
lsof -i :8080
# Or change PORT variable in test plan
```

#### JWT Token Not Found
```
Error: JWT_TOKEN variable not found
```
**Solution**: 
- Ensure login request runs before authenticated requests
- Check JSON extractor configuration
- Verify login response includes token

#### High Error Rate
```
Error %: 50.00%
```
**Solution**:
- Check JMeter log for specific errors
- Reduce thread count and ramp-up time
- Verify database connections
- Check server logs

#### Out of Memory (JMeter)
```
Error: Java heap space
```
**Solution**: Increase JMeter heap size:
```bash
export HEAP="-Xms1g -Xmx4g"
jmeter -n -t fingest_performance_test.jmx -l results.jtl
```

#### Database Connection Pool Exhausted
```
Error: Connection pool exhausted
```
**Solution**:
- Increase database connection pool size
- Reduce thread count
- Add delays between requests

### Debugging Tests

1. **Enable View Results Tree** in GUI mode
2. **Check request/response data**:
   - Request tab: Verify request data
   - Response data tab: Check server response
   - Assertion results tab: See assertion failures

3. **Enable debug logging**:
   ```bash
   jmeter -n -t test.jmx -l results.jtl -Jlog_level.jmeter=DEBUG
   ```

4. **Use BeanShell/JSR223 samplers** for debugging:
   ```java
   log.info("Variable value: " + vars.get("WALLET_ID"));
   ```

### Getting Help

- **JMeter Documentation**: https://jmeter.apache.org/usermanual/
- **JMeter Best Practices**: https://jmeter.apache.org/usermanual/best-practices.html
- **FinGest API Documentation**: See `/docs/api_examples.md`
- **GitHub Issues**: Report issues on the project repository

## Advanced Topics

### Distributed Testing

Run tests across multiple machines:

1. **Start JMeter server** on remote machines:
   ```bash
   jmeter-server
   ```

2. **Configure remote hosts** in `jmeter.properties`:
   ```properties
   remote_hosts=192.168.1.10,192.168.1.11
   ```

3. **Run distributed test**:
   ```bash
   jmeter -n -t test.jmx -r -l results.jtl
   ```

### Real-time Monitoring with InfluxDB

1. **Start InfluxDB**:
   ```bash
   docker run -d -p 8086:8086 influxdb:1.8
   ```

2. **Create database**:
   ```bash
   influx -execute 'CREATE DATABASE jmeter'
   ```

3. **Enable Backend Listener** in test plan

4. **Visualize with Grafana**:
   ```bash
   docker run -d -p 3000:3000 grafana/grafana
   ```

### CI/CD Integration

Example GitHub Actions workflow:

```yaml
name: Performance Tests

on: [push]

jobs:
  performance-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Start API
        run: docker compose up -d
      
      - name: Install JMeter
        run: |
          wget https://archive.apache.org/dist/jmeter/binaries/apache-jmeter-5.6.3.tgz
          tar -xzf apache-jmeter-5.6.3.tgz
      
      - name: Run Tests
        run: |
          apache-jmeter-5.6.3/bin/jmeter -n \
            -t tests/fingest_performance_test.jmx \
            -l results.jtl \
            -e -o report
      
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: jmeter-results
          path: report/
```

## Conclusion

This JMeter test plan provides comprehensive coverage of the FinGest API, including:

- ✅ All 16 API endpoints tested
- ✅ Functional validation with assertions
- ✅ Error case testing
- ✅ Performance benchmarking capabilities
- ✅ Load and stress testing scenarios
- ✅ Realistic test data generation
- ✅ Detailed reporting and analysis

For questions or issues, please refer to the main project documentation or open an issue on GitHub.

---

**Version**: 1.0.0  
**Last Updated**: 2025-11-29  
**JMeter Version**: 5.6.3+  
**API Version**: Compatible with FinGest v0.1.0
