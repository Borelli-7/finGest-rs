# Authentication Service Documentation

## Overview

The Money Manager API now includes a robust authentication service that provides secure user registration, login, and JWT-based authorization. The implementation follows OWASP security best practices.

## Security Features

### Password Hashing
- **Algorithm**: bcrypt with default cost factor (12)
- **Storage**: Passwords are never stored in plain text
- **Validation**: Minimum 8 characters required

### JWT Tokens
- **Algorithm**: HS256 (HMAC with SHA-256)
- **Expiration**: Configurable (default: 24 hours)
- **Claims**: User login, admin flag, issued at, and expiration time

### OWASP Compliance
- Secure password storage using bcrypt
- JWT tokens for stateless authentication
- Protected endpoints with middleware
- Input validation on all endpoints
- Error messages don't leak sensitive information

## Configuration

Add the following to your `.env` file:

```env
# JWT Authentication
JWT_SECRET=your-secret-key-min-32-characters-change-in-production
JWT_EXPIRATION_HOURS=24
```

**Important**: Generate a secure random secret for production:
```bash
openssl rand -base64 32
```

## API Endpoints

### 1. User Registration

**Endpoint**: `POST /api/auth/register`

**Request Body**:
```json
{
  "login": "john_doe",
  "firstName": "John",
  "lastName": "Doe",
  "password": "SecurePassword123!",
  "admin": false
}
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

**Validation Rules**:
- `login`: Required, minimum 1 character
- `password`: Required, minimum 8 characters
- `firstName`, `lastName`: Optional
- `admin`: Optional, defaults to `false`

**Errors**:
- `400 Bad Request`: Validation error or user already exists
- `500 Internal Server Error`: Database or server error

### 2. User Login

**Endpoint**: `POST /api/auth/login`

**Request Body**:
```json
{
  "login": "john_doe",
  "password": "SecurePassword123!"
}
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

**Errors**:
- `401 Unauthorized`: Invalid credentials
- `400 Bad Request`: Validation error

### 3. Token Verification

**Endpoint**: `GET /api/auth/verify`

**Headers**:
```
Authorization: Bearer <token>
```

**Response** (200 OK):
```json
{
  "valid": true,
  "login": "john_doe",
  "admin": false
}
```

**Errors**:
- `401 Unauthorized`: Missing or invalid token

## Using Protected Routes

### JWT Middleware

To protect routes, use the `JwtAuth` middleware:

```rust
use money_manager_api::api::middleware::JwtAuth;

App::new()
    .wrap(JwtAuth::new(jwt_secret))
    .route("/api/protected", web::get().to(handler))
```

### Accessing Claims in Handlers

```rust
use money_manager_api::api::middleware::get_claims_from_request;
use money_manager_api::services::auth_service::Claims;

async fn protected_handler(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let claims = get_claims_from_request(&req)
        .ok_or_else(|| AppError::AuthenticationError("No claims found".to_string()))?;
    
    // Access user information
    let user_login = claims.sub;
    let is_admin = claims.admin;
    
    // ... your handler logic
}
```

## Client Integration

### Registration Flow

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

### Login Flow

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "john_doe",
    "password": "SecurePassword123!"
  }'
```

### Using the Token

```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

curl -X GET http://localhost:8080/api/auth/verify \
  -H "Authorization: Bearer $TOKEN"
```

### Accessing Protected Resources

```bash
curl -X GET http://localhost:8080/api/users \
  -H "Authorization: Bearer $TOKEN"
```

## Security Best Practices

### Production Deployment

1. **JWT Secret**: Use a strong, randomly generated secret (32+ characters)
   ```bash
   openssl rand -base64 32
   ```

2. **HTTPS**: Always use HTTPS in production to protect tokens in transit

3. **Token Storage**: 
   - Web: Store tokens in httpOnly cookies or secure storage
   - Mobile: Use secure keychain/keystore

4. **Token Expiration**: Set appropriate expiration times
   - Short-lived tokens (1-24 hours) for better security
   - Implement refresh token mechanism for better UX

5. **Password Policy**: Enforce strong passwords
   - Minimum 8 characters (current)
   - Consider adding complexity requirements
   - Implement rate limiting on login attempts

6. **CORS**: Configure CORS properly for your frontend
   ```rust
   let cors = Cors::default()
       .allowed_origin("https://yourdomain.com")
       .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
       .allowed_headers(vec!["Authorization", "Content-Type"])
       .supports_credentials();
   ```

### Monitoring and Logging

- Log authentication attempts (without passwords)
- Monitor failed login attempts
- Set up alerts for suspicious activity
- Regularly rotate JWT secrets

## Testing

The auth service includes comprehensive unit tests:

```bash
cargo test
```

Test coverage includes:
- Password hashing and verification
- JWT token generation and validation
- User registration
- Middleware authentication

## References

- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
- [JWT Best Practices](https://datatracker.ietf.org/doc/html/rfc8725)
- [bcrypt specification](https://en.wikipedia.org/wiki/Bcrypt)
