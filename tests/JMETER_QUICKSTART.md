# JMeter Quick Start Guide

> Get started with JMeter performance testing for FinGest API in 5 minutes

## Prerequisites

- Java 8 or later installed
- FinGest API running locally (or accessible remotely)
- Basic understanding of HTTP APIs

## Step 1: Install Apache JMeter (2 minutes)

### On Linux/Mac:
```bash
# Download JMeter
wget https://archive.apache.org/dist/jmeter/binaries/apache-jmeter-5.6.3.tgz

# Extract
tar -xzf apache-jmeter-5.6.3.tgz
cd apache-jmeter-5.6.3

# Verify installation
bin/jmeter --version
```

### On Windows:
1. Download from: https://jmeter.apache.org/download_jmeter.cgi
2. Extract the ZIP file
3. Open Command Prompt and run: `bin\jmeter.bat --version`

## Step 2: Start the FinGest API (1 minute)

```bash
# Navigate to project directory
cd /path/to/finGest-rs

# Start with Docker (recommended)
docker compose up -d

# OR start with Cargo
cargo run --release
```

Verify the API is running:
```bash
curl http://localhost:8080/resources/categories
```

## Step 3: Run Your First Test (2 minutes)

### Option A: GUI Mode (For Learning)

```bash
# Navigate to JMeter directory
cd apache-jmeter-5.6.3

# Start JMeter GUI
bin/jmeter

# In JMeter:
# 1. File → Open → Select: /path/to/finGest-rs/tests/fingest_performance_test.jmx
# 2. Click the green "Start" button (or press Ctrl+R)
# 3. Watch the tests run in the listeners
```

**Tip**: Open "View Results Tree" to see detailed request/response data.

### Option B: CLI Mode (For Real Testing)

```bash
# Run tests and generate HTML report
apache-jmeter-5.6.3/bin/jmeter -n \
  -t /path/to/finGest-rs/tests/fingest_performance_test.jmx \
  -l results.jtl \
  -e -o report/

# View the report
open report/index.html   # Mac
xdg-open report/index.html   # Linux
start report/index.html  # Windows
```

## Understanding Your Results

### Key Metrics to Watch

1. **Samples**: Number of requests executed
2. **Average Response Time**: Should be < 200ms for most endpoints
3. **Error %**: Should be 0% for functional tests
4. **Throughput**: Requests per second

### Success Criteria

✅ **Good Performance**:
- Error Rate: 0%
- Average Response Time: < 200ms
- All assertions passed

❌ **Needs Investigation**:
- Error Rate: > 0%
- Response Time: > 500ms
- Failed assertions

## What's Being Tested?

The test plan includes 7 thread groups:

1. ✅ **Authentication Flow** (10 users) - Login, registration
2. ✅ **Categories and Users** (5 users) - Basic GET operations
3. ✅ **Wallet Operations** (10 users) - Wallet CRUD
4. ✅ **Expense Operations** (15 users) - Expense tracking
5. ✅ **Budget Operations** (8 users) - Budget management
6. ⚪ **Load Test** (50 users) - Disabled by default
7. ⚪ **Stress Test** (100 users) - Disabled by default

**Total**: 48 concurrent users testing 16 API endpoints with 24 test scenarios

## Common Issues

### "Connection Refused"
**Problem**: API is not running  
**Solution**: Start the API with `docker compose up -d`

### "Error rate is high"
**Problem**: Tests running too fast or database issue  
**Solution**: 
- Check API logs: `docker compose logs api`
- Reduce thread count in test plan
- Verify database is accessible

### "JMeter not found"
**Problem**: JMeter not in PATH  
**Solution**: Use full path to JMeter binary: `/path/to/apache-jmeter-5.6.3/bin/jmeter`

## Next Steps

### 1. Customize for Your Environment

Edit variables in the test plan:
- Open in JMeter GUI
- Navigate to: Test Plan → User Defined Variables
- Change `BASE_URL`, `PORT`, `PROTOCOL` as needed
- Save the test plan

### 2. Run Load Tests

Enable the load test (disabled by default):
```bash
# In JMeter GUI:
# Right-click "06 - Load Test - Concurrent Operations"
# Select "Enable"
# Save and run
```

### 3. Generate Custom Reports

```bash
# Run with custom parameters
jmeter -n -t fingest_performance_test.jmx \
  -JBASE_URL=api.example.com \
  -JPORT=443 \
  -JPROTOCOL=https \
  -l results.jtl \
  -e -o report/
```

### 4. Integrate with CI/CD

Add to your GitHub Actions, Jenkins, or other CI/CD pipeline:
```yaml
- name: Run Performance Tests
  run: |
    jmeter -n -t tests/fingest_performance_test.jmx \
      -l results.jtl \
      -e -o report/
    
- name: Upload Report
  uses: actions/upload-artifact@v3
  with:
    name: jmeter-report
    path: report/
```

## Learn More

- 📖 **Detailed Guide**: See [JMETER_TESTING_GUIDE.md](JMETER_TESTING_GUIDE.md)
- 📊 **API Coverage**: See [API_ENDPOINT_COVERAGE.md](API_ENDPOINT_COVERAGE.md)
- ✅ **Validation Report**: See [JMETER_VALIDATION_REPORT.md](JMETER_VALIDATION_REPORT.md)
- 🌐 **JMeter Docs**: https://jmeter.apache.org/usermanual/

## Tips for Success

1. **Always run performance tests in CLI mode** (not GUI)
2. **Start small**: Run with default settings first
3. **Monitor resources**: Watch CPU, memory, database connections
4. **Run multiple times**: Ensure consistent results
5. **Clean up**: Remove test data after tests

## Need Help?

- Check the [troubleshooting section](JMETER_TESTING_GUIDE.md#troubleshooting)
- Review the [full testing guide](JMETER_TESTING_GUIDE.md)
- Open an issue on GitHub
- Review JMeter documentation

---

**You're ready to go!** 🚀

Run the tests and start optimizing your API performance.
