# Rust Clean Architecture Template

A production-ready microservice template built with Rust, designed for scalability, maintainability, and type safety using Clean Architecture principles.

## Purpose

This project serves as a robust starting point for building backend microservices in Rust. It enforces a strict separation of concerns, ensuring that business logic remains independent of frameworks, databases, and external interfaces.

## Key Features

- Strict Clean Architecture: Explicit layers for Domain, Application, Infrastructure, and Presentation.
- Type-Safe Database: PostgreSQL integration via SQLx for compile-time query verification.
- High Performance: Built on Actix-web and Tokio.
- Event-Driven: Ready-to-use RabbitMQ integration for asynchronous messaging.
- Security First: JWT Authentication (Access + Refresh Tokens) with persisted refresh tokens (rotation) and Bcrypt password hashing.
- Brute-force Protection: Per-IP rate limiting on login plus failed login tracking and temporary account lockout (PostgreSQL-backed).
- Developer Experience: Pre-configured with Docker Compose, Flyway migrations, and Swagger/OpenAPI documentation.

## Prerequisites

- Rust (1.75+)
- Docker and Docker Compose
- Flyway CLI (for database migrations)
- Make (optional, for convenience scripts)

## Quick Start

### 1. Setup Environment

```bash
cp .env.example .env
```

Update `.env` with your credentials.

### 2. Start Infrastructure

```bash
make setup
```

This command starts Docker containers, runs migrations, and builds the project.

### 3. Run the API

```bash
make run-api
```

### 4. Access

- API Server: http://localhost:8080
- Swagger UI: http://localhost:8080/swagger-ui/

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| DATABASE_URL | Yes | - | PostgreSQL connection string |
| JWT_SECRET | Yes | - | Secret key for JWT signing |
| JWT_EXPIRY_HOURS | No | 24 | Access token expiry in hours |
| CORS_ALLOWED_ORIGINS | No | http://localhost:3000 | Comma-separated list of allowed origins |
| API_HOST | No | 0.0.0.0 | Server bind host |
| API_PORT | No | 8080 | Server bind port |

## Project Structure

The workspace is organized into modular crates to enforce dependency rules:

```
crates/
  domain/           - Pure business logic (Entities, Interfaces) - No external deps
  application/      - Use Cases and DTOs - Depends only on Domain
  infrastructure/   - Database, Auth, Adapters - Implements Domain Interfaces
  presentation/     - API Handlers and Consumers - Entry points
bins/              - Executables (api, consumer) - Wiring everything together
```

## Commands

### Build

```bash
make build
# or
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run specific package tests
cargo test -p rust-clean-application

# Run single test
cargo test -p rust-clean-application -- test_name

# Run with logs
RUST_LOG=debug cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

### Running

```bash
# Run API server
make run-api
# or
cargo run --bin api

# Run RabbitMQ consumer
make run-consumer
# or
cargo run --bin consumer
```

## API Endpoints

### Authentication

| Method | Path | Description |
|--------|------|-------------|
| POST | /auth/register | Register new user |
| POST | /auth/login | Login (includes brute-force check) |
| POST | /auth/refresh | Refresh access token |

### Users (requires JWT)

| Method | Path | Description |
|--------|------|-------------|
| GET | /users | List all users |
| GET | /users/{id} | Get user by ID |
| PUT | /users/{id} | Update user |
| DELETE | /users/{id} | Delete user |

## Database Migrations (Flyway)

Migrations are located in `./migrations/`.

Included migrations:
- V1__create_users_table.sql: users table
- V2__add_jwt_table.sql: refresh token persistence
- V3__add_failed_logins_table.sql: failed login tracking and lockout
- V4__failed_logins_created_at_not_null.sql: enforce created_at non-null

### Flyway Commands

```bash
# Apply all migrations
flyway -url="jdbc:postgresql://localhost:5432/rust_clean_db" \
       -user=postgres -password=postgres \
       -locations=filesystem:./migrations migrate

# Repair (fix checksum mismatches)
flyway -url="jdbc:postgresql://localhost:5432/rust_clean_db" \
       -user=postgres -password=postgres \
       -locations=filesystem:./migrations repair
```

## Security Configuration

### Brute-force Protection

The system implements two layers of protection:

1. Rate Limiting (in-memory): Limits requests per IP per minute
2. Account Lockout (database): Locks account after failed login attempts

Environment variables:
- BRUTE_FORCE_MAX_LOGIN_ATTEMPTS: Number of failed attempts before lockout (default: 5)
- BRUTE_FORCE_LOCKOUT_DURATION_MINUTES: Lockout duration in minutes (default: 30)
- BRUTE_FORCE_RATE_LIMIT_REQUESTS_PER_MINUTE: Rate limit per IP (default: 10)

### Password Hashing

Passwords are hashed using Bcrypt with cost factor 12.

### JWT

- Access tokens: Short-lived, stored in memory
- Refresh tokens: Long-lived, persisted in database with hash

## Architecture Notes

This project follows Clean Architecture principles with strict dependency rules:

- Domain layer has no external dependencies
- Application layer depends only on Domain
- Infrastructure layer implements Domain interfaces
- Presentation layer depends only on Application (not Domain directly)
- Binary (main.rs) wires everything together
