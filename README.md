# Rust Clean Architecture Microservice Template

A production-ready microservice template built with Rust, implementing **Clean Architecture** principles with a focus on scalability, maintainability, and type safety.

## 🎯 Project Overview

This template demonstrates best practices for building backend microservices in Rust using:
- **Clean Architecture**: Separation of concerns across domain, application, and infrastructure layers
- **Type-Safe Database Access**: SQLx with compile-time query verification
- **Event-Driven Architecture**: RabbitMQ integration for async communication
- **JWT Authentication**: Secure token-based authentication with refresh tokens
- **PostgreSQL**: Reliable data persistence with Flyway migrations
- **REST API**: Actix-web with comprehensive error handling
- **API Documentation**: Auto-generated Swagger/OpenAPI specs with Utoipa

## 📁 Project Structure

```
rust-clean-template/
├── crates/
│   ├── domain/              # Core business logic (entities, traits, errors)
│   ├── application/         # Use cases, services, and DTOs
│   ├── infrastructure/      # External integrations (database, messaging)
│   └── presentation/
│       ├── api/            # HTTP API handlers and routes
│       └── consumer/       # Event consumer handlers
├── bins/
│   ├── api/                # API server executable
│   └── consumer/           # Message consumer executable
├── migrations/             # Database migrations (Flyway)
├── scripts/                # Utility scripts
└── docker-compose.yml      # Local development environment
```

### Architecture Layers

#### Domain Layer (`crates/domain`)
- **User Entity**: User model with role-based access control
- **UserRole Enum**: Admin, User roles for RBAC
- **Repository Trait**: Abstract repository pattern for data access
- **Error Types**: Domain-specific error handling with `DomainError`

#### Application Layer (`crates/application`)
- **AuthService**: Registration, login, token refresh, token validation
- **UserService**: User CRUD operations and updates
- **DTOs**: Request/Response models with validation
- **Business Logic**: Core use case implementations

#### Infrastructure Layer (`crates/infrastructure`)
- **Database**: PostgreSQL connection pool with SQLx
- **PgUserRepository**: SQL implementation of UserRepository
- **Messaging**: RabbitMQ producer and consumer setup

#### Presentation Layer
- **API** (`crates/presentation/api`): HTTP handlers and routes with Actix-web
- **Consumer** (`crates/presentation/consumer`): Event processing with proper logging

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ ([Install](https://rustup.rs/))
- Docker & Docker Compose
- PostgreSQL 15 (via Docker)
- RabbitMQ 3.12 (via Docker)

### Setup

1. **Clone and configure**
```bash
# Copy environment variables
cp .env.example .env

# Update .env with your configuration
# Important: JWT_SECRET must be at least 32 characters
```

2. **Start infrastructure**
```bash
docker-compose up -d
```

3. **Run migrations**
```bash
make migrate
```

4. **Build project**
```bash
make build
```

5. **Run API server**
```bash
make run-api
# API available at http://localhost:8080
# Swagger UI at http://localhost:8080/swagger-ui/
```

6. **Run message consumer** (in another terminal)
```bash
make run-consumer
```

## 📡 API Endpoints

### Authentication

#### Register User
```bash
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123",
  "name": "John Doe"
}
```

**Response**: `201 Created`
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "User",
  "created_at": "2024-01-01T12:00:00Z"
}
```

#### Login
```bash
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response**: `200 OK`
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "refresh_token": "random_refresh_token_string",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

#### Refresh Token
```bash
POST /auth/refresh
Authorization: Bearer {access_token}
Content-Type: application/json

{
  "refresh_token": "random_refresh_token_string"
}
```

**Response**: `200 OK`
```json
{
  "access_token": "new_access_token",
  "refresh_token": "new_refresh_token",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### User Management (Protected - Requires Authentication)

#### Get All Users
```bash
GET /users
Authorization: Bearer {access_token}
```

**Response**: `200 OK`
```json
[
  {
    "id": "uuid",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "User",
    "created_at": "2024-01-01T12:00:00Z"
  }
]
```

#### Get User by ID
```bash
GET /users/{id}
Authorization: Bearer {access_token}
```

#### Update User
```bash
PUT /users/{id}
Authorization: Bearer {access_token}
Content-Type: application/json

{
  "name": "Jane Doe"
}
```

**Response**: `200 OK`
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "name": "Jane Doe",
  "role": "User",
  "created_at": "2024-01-01T12:00:00Z"
}
```

#### Delete User
```bash
DELETE /users/{id}
Authorization: Bearer {access_token}
```

**Response**: `204 No Content`

## 🧪 Testing

Run all tests:
```bash
# Unit tests
make test

# Full test suite
cargo test --all
```

## 📚 Configuration

Environment variables (`.env`):

```env
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/rust_clean_db
DATABASE_POOL_MAX_SIZE=5

# JWT
JWT_SECRET=your-secret-key-min-32-characters-long
JWT_EXPIRY_HOURS=24

# RabbitMQ
RABBITMQ_URL=amqp://guest:guest@localhost:5672
RABBITMQ_QUEUE=rust_clean_queue
RABBITMQ_EXCHANGE=rust_clean_exchange

# API Server
API_HOST=0.0.0.0
API_PORT=8080

# Logging
RUST_LOG=info
```

## 🏗️ Architecture Decisions

### Why Clean Architecture?
- **Maintainability**: Each layer has a single responsibility
- **Testability**: Business logic isolated from infrastructure
- **Flexibility**: Easy to swap database, messaging, or API framework
- **Scalability**: Clear boundaries make the codebase easy to extend

### Why These Technologies?
- **Rust**: Type safety, memory safety, and performance
- **Actix-web**: High-performance async web framework
- **SQLx**: Compile-time verified SQL queries prevent runtime errors
- **RabbitMQ**: Reliable message broker for event-driven features
- **PostgreSQL**: Battle-tested relational database

## 🔒 Security Features

- ✅ **Password Hashing**: bcrypt with 12 rounds
- ✅ **JWT Tokens**: HS256 algorithm for token signing
- ✅ **Refresh Tokens**: Separate refresh tokens for enhanced security
- ✅ **CORS**: Configurable CORS middleware
- ✅ **Input Validation**: Validator crate for request validation

## 📊 Performance Considerations

- Connection pooling for database
- Async/await for non-blocking I/O
- Zero-copy middleware chains
- Compiled queries with SQLx

## 🔄 Event-Driven Features

The consumer service processes three types of user events:
- `user.created`: Triggered when a new user registers
- `user.updated`: Triggered when user profile changes
- `user.deleted`: Triggered when user account is removed

Events are logged and can be extended with custom processing logic.

## 🛠️ Development Helpers

```bash
# Build everything
make build

# Build and start API
make run-api

# Start message consumer
make run-consumer

# Run database migrations
make migrate

# Run tests
make test

# Run linting
make lint

# Generate documentation
make doc

# Clean build artifacts
make clean

# Full setup (migrations + build + run)
make setup
```

## 📋 API Documentation

Complete API documentation is available via Swagger UI after starting the API server:
- **Swagger UI**: http://localhost:8080/swagger-ui/
- **OpenAPI Spec**: http://localhost:8080/api-docs/openapi.json

## 🐳 Docker Deployment

Build and run using Docker:

```bash
# Build API container
docker build -f Dockerfile -t rust-clean-api .

# Run container
docker run -p 8080:8080 \
  -e DATABASE_URL=postgresql://... \
  -e JWT_SECRET=your-secret \
  rust-clean-api
```

## 📈 Monitoring & Logging

The project uses `tracing` for structured logging:

```bash
# Set log level
export RUST_LOG=debug
make run-api
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`

## 🤝 Contributing

Follow these guidelines when extending the template:

1. **Add tests** for new features in the same file with `#[cfg(test)]`
2. **Keep domain logic separate** from infrastructure code
3. **Use DTOs** for request/response serialization
4. **Document public APIs** with doc comments
5. **Follow Rust naming conventions** and style guidelines

## 📝 License

This template is provided as-is for educational and commercial use.

## 🔗 Related Resources

- [Clean Architecture by Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Actix-web Documentation](https://actix.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)

---

**Last Updated**: 2024
**Rust Version**: 1.70+
**Status**: Production Ready ✅
