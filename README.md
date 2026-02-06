# Rust Clean Architecture Template

A production-ready microservice template built with Rust, designed for scalability, maintainability, and type safety using **Clean Architecture** principles.

## Purpose

This project serves as a robust starting point for building backend microservices in Rust. It enforces a strict separation of concerns, ensuring that business logic remains independent of frameworks, databases, and external interfaces.

## Key Features

- **Strict Clean Architecture**: Explicit layers for Domain, Application, Infrastructure, and Presentation.
- **Type-Safe Database**: PostgreSQL integration via **SQLx** for compile-time query verification.
- **High Performance**: Built on **Actix-web** and **Tokio**.
- **Event-Driven**: Ready-to-use **RabbitMQ** integration for asynchronous messaging.
- **Security First**: JWT Authentication (Access + Refresh Tokens) and Bcrypt password hashing.
- **Developer Experience**: Pre-configured with **Docker Compose**, **Flyway** migrations, and **Swagger/OpenAPI** documentation.

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.75+)
- Docker & Docker Compose
- Make (optional, for convenience scripts)

### Running the Project

1.  **Setup Environment**
    ```bash
    cp .env.example .env
    # Update .env with your credentials if needed
    ```

2.  **Start Infrastructure & Run**
    ```bash
    make setup
    # This command starts Docker containers, runs migrations, and builds the project.
    ```

3.  **Access**
    - **API Server**: `http://localhost:8080`
    - **Swagger UI**: `http://localhost:8080/swagger-ui/`

## Project Structure

The workspace is organized into modular crates to enforce dependency rules:

```
├── crates/
│   ├── domain/           # Pure business logic (Entities, Interfaces) - No external deps
│   ├── application/      # Use Cases & DTOs - Depends only on Domain
│   ├── infrastructure/   # Database, Auth, Adapters - Implements Domain Interfaces
│   └── presentation/     # API Handlers & Consumers - Entry points
├── bins/                 # Executables (api, consumer) - Wiring everything together
```

## Commands

| Command | Description |
|---------|-------------|
| `make build` | Build the project |
| `make test` | Run unit tests |
| `make run-api` | Start the REST API server |
| `make run-consumer` | Start the RabbitMQ consumer |
| `make lint` | Run clippy and format checks |

## Database Migrations (Flyway)

Prerequisites: [Flyway CLI](https://flywaydb.org/documentation/usage/commands/) installed.

Migrations location: `./migrations/`

```bash
# Apply all migrations
flyway -url="postgresql://postgres:postgres@localhost:5432/rust_clean_db" migrate

# Create new migration
flyway -url="..." create <name>

# Repair (fix checksum mismatches)
flyway -url="..." repair
```
