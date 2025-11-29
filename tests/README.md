# FinGest API Testing Documentation

This directory contains comprehensive testing resources for the FinGest Money Manager API, including unit tests, integration tests, Postman collections, and Apache JMeter performance testing.

## 📁 Testing Resources

### Integration Tests
- **`api_tests.rs`** - Rust integration tests for the API
  - Tests all major API functionality
  - Uses mock services for isolated testing
  - Run with: `cargo test --test api_tests`

### Postman Testing
- **`postman_collection.json`** - Complete Postman API test collection
- **`postman_environment_local.json`** - Local environment configuration
- **`POSTMAN_TESTING_GUIDE.md`** - Detailed guide for Postman testing
  - 27 requests covering all endpoints
  - Automated test scripts
  - Error case validation

### Apache JMeter Performance Testing
- **`fingest_performance_test.jmx`** - Comprehensive JMeter test plan
- **`JMETER_TESTING_GUIDE.md`** - Complete JMeter testing guide
- **`API_ENDPOINT_COVERAGE.md`** - Endpoint documentation and coverage
- **`JMETER_VALIDATION_REPORT.md`** - Test plan validation report

## 🚀 Quick Start

### Run Unit and Integration Tests
```bash
# All tests
cargo test

# Integration tests only
cargo test --test api_tests

# With output
cargo test -- --nocapture
```

### Run Postman Tests
1. Import `postman_collection.json` and `postman_environment_local.json` into Postman
2. Select "FinGest Local Environment"
3. Start the API: `docker compose up -d`
4. Run the collection

See [`POSTMAN_TESTING_GUIDE.md`](POSTMAN_TESTING_GUIDE.md) for details.

### Run JMeter Performance Tests
```bash
# Install JMeter (requires Java 8+)
wget https://archive.apache.org/dist/jmeter/binaries/apache-jmeter-5.6.3.tgz
tar -xzf apache-jmeter-5.6.3.tgz

# Start the API
docker compose up -d

# Run tests
apache-jmeter-5.6.3/bin/jmeter -n \
  -t tests/fingest_performance_test.jmx \
  -l results.jtl \
  -e -o report/

# View results
open report/index.html
```

See [`JMETER_TESTING_GUIDE.md`](JMETER_TESTING_GUIDE.md) for details.

## 📊 Test Coverage

### API Endpoints Coverage
- **16/16 endpoints** (100% coverage)
- **Authentication**: 3 endpoints
- **Categories**: 1 endpoint
- **Users**: 2 endpoints
- **Wallets**: 3 endpoints
- **Expenses**: 5 endpoints
- **Budgets**: 2 endpoints

### Test Scenarios
- **Integration Tests**: Core functionality with mocks
- **Postman Tests**: 27 requests with automated validation
- **JMeter Tests**: 24 scenarios (19 success + 5 error cases)

## 📖 Documentation

### For Developers
- [`POSTMAN_TESTING_GUIDE.md`](POSTMAN_TESTING_GUIDE.md) - Interactive API testing
- Integration tests in `api_tests.rs` - Code-level testing

### For QA and Performance Testing
- [`JMETER_TESTING_GUIDE.md`](JMETER_TESTING_GUIDE.md) - Performance testing guide
- [`API_ENDPOINT_COVERAGE.md`](API_ENDPOINT_COVERAGE.md) - Complete endpoint reference
- [`JMETER_VALIDATION_REPORT.md`](JMETER_VALIDATION_REPORT.md) - Test plan validation

## 🎯 Testing Types

### 1. Unit/Integration Tests (Rust)
- **Purpose**: Validate core business logic
- **When**: During development, before commits
- **Tools**: Cargo test, mockall
- **Speed**: Fast (< 1 second)

### 2. Postman Tests
- **Purpose**: Manual and automated API testing
- **When**: Feature development, API exploration
- **Tools**: Postman
- **Speed**: Fast to moderate

### 3. JMeter Performance Tests
- **Purpose**: Performance, load, and stress testing
- **When**: Before releases, performance benchmarking
- **Tools**: Apache JMeter
- **Speed**: Varies (functional: seconds, load: minutes)

#### JMeter Test Types
- **Functional Tests** (Groups 1-5): Validate all endpoints work correctly
- **Load Tests** (Group 6): 50 concurrent users, 5 minutes
- **Stress Tests** (Group 7): 100 concurrent users, 10 minutes

## 🔍 Which Testing Tool to Use?

### Use Cargo Tests When:
- ✅ Developing new features
- ✅ Testing business logic
- ✅ Running CI/CD pipelines
- ✅ Validating code changes

### Use Postman When:
- ✅ Exploring the API
- ✅ Manual testing during development
- ✅ Creating example requests
- ✅ Sharing API usage with team
- ✅ Quick validation of endpoints

### Use JMeter When:
- ✅ Performance benchmarking
- ✅ Load testing before deployment
- ✅ Stress testing to find limits
- ✅ Automated performance regression testing
- ✅ Validating SLAs and performance targets

## 🎓 Learning Resources

### Postman
- [Official Postman Documentation](https://learning.postman.com/)
- [`POSTMAN_TESTING_GUIDE.md`](POSTMAN_TESTING_GUIDE.md) - Project-specific guide

### Apache JMeter
- [Official JMeter Documentation](https://jmeter.apache.org/usermanual/)
- [JMeter Best Practices](https://jmeter.apache.org/usermanual/best-practices.html)
- [`JMETER_TESTING_GUIDE.md`](JMETER_TESTING_GUIDE.md) - Project-specific guide

### Rust Testing
- [Rust Book - Testing Chapter](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Actix-web Testing](https://actix.rs/docs/testing/)

## 🚨 Important Notes

### Before Running Tests
1. **Start the API**: `docker compose up -d` or `cargo run`
2. **Verify connectivity**: `curl http://localhost:8080/resources/categories`
3. **Check database**: Ensure PostgreSQL is running and accessible

### Load/Stress Testing
- ⚠️ Only run against **test environments**
- ⚠️ Load and stress tests are **disabled by default** in JMeter
- ⚠️ Monitor system resources during tests
- ⚠️ Clean up test data after tests

### Performance Targets
- Simple GET: < 100ms
- POST/PUT: < 200ms
- Complex queries: < 500ms
- Error rate: 0% for functional tests

## 📝 Contributing

When adding new endpoints or features:
1. ✅ Add integration tests in `api_tests.rs`
2. ✅ Add Postman requests to the collection
3. ✅ Add JMeter samplers to the test plan
4. ✅ Update documentation accordingly
5. ✅ Run all tests before submitting PR

## 🔗 Related Documentation

- [Main README](../README.md) - Project overview
- [API Examples](../docs/api_examples.md) - API usage examples
- [Authentication Guide](../docs/authentication.md) - Auth implementation
- [Architecture Overview](../docs/money_manager_architecture.md) - System design

## 📧 Support

Questions or issues with testing?
- Check the relevant testing guide
- Review the validation report
- Open an issue on GitHub
- Review existing test cases for examples

---

**Last Updated**: 2025-11-29  
**Test Coverage**: 100% (16/16 endpoints)  
**Testing Tools**: Cargo Test, Postman, Apache JMeter
