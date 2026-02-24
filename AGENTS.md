# Operational Guidelines for Agents

This document defines the strict protocols, coding standards, and architectural rules for agents operating within this repository. Adherence is mandatory to maintain the integrity of the Rust Clean Architecture.

## 1. Project Architecture (Strict Clean Architecture)

The workspace is organized into explicit layers. You must respect the Dependency Rule: Source code dependencies must point only inward, toward higher-level policies.

### Structure & Dependencies
- `crates/domain` (Core):
  - Contains: Entities, Value Objects, Domain Errors, Repository Interfaces (Traits).
  - Dependencies: NONE. Pure Rust types only.
- `crates/application` (Use Cases):
  - Contains: Use Cases (Interactors), DTOs, Input/Output Ports (Abstractions).
  - Dependencies: `domain`.
  - RESTRICTION: No dependencies on `infrastructure` or `presentation`. No concrete `bcrypt`, `jwt`, or `sqlx` logic here. Use abstractions defined in `abstractions/` module.
- `crates/presentation` (Interface Adapters):
  - Contains: API Handlers (REST), Event Consumers (Workers), Middleware.
  - Dependencies: `application` only. **MUST NOT import domain directly** except for error conversion (e.g., `impl From<DomainError> for ApiError`).
  - RESTRICTION: No direct dependency on `infrastructure`. Do not import `sqlx` or database pools directly. Interact via Application Use Cases and Abstractions.
- `crates/infrastructure` (Frameworks & Drivers):
  - Contains: Database implementations (Sqlx), Security adapters (Bcrypt, JWT), External APIs.
  - Dependencies: `domain`, `application` (to implement traits).
- `bins/` (Composition Root):
  - Contains: `main.rs` entry points (`api`, `consumer`).
  - Dependencies: All layers. Responsible for wiring traits to implementations (Dependency Injection).

## 2. Development Workflow & Commands

### Environment Setup
```bash
# Copy environment file
cp .env.example .env

# Database must be running (PostgreSQL on localhost:5432)
# Check docker-compose.yml for services
```

### Build & Check
- Check workspace: `cargo check --workspace`
- Build release: `cargo build --release`

### Testing
- Run all tests: `cargo test --workspace`
- Run specific package tests: `cargo test -p rust-clean-application`
- Run single test case: `cargo test -p <package> -- <test_name>`
- Run with logs: `RUST_LOG=debug cargo test -- --nocapture`
- Run integration tests (requires real database):
  ```bash
  DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_clean_db" \
    cargo test -p rust-clean-infrastructure
  ```

### Code Quality (Mandatory)
After every edit, you must execute:
1. Format: `cargo fmt --all`
2. Lint: `cargo clippy --all-targets --all-features -- -D warnings`
3. Test: `cargo test --workspace`

### Database Migrations (Flyway)
```bash
# Apply migrations (requires Flyway CLI installed)
flyway -url="jdbc:postgresql://localhost:5432/rust_clean_db" \
       -user=postgres -password=postgres \
       -locations=filesystem:./migrations migrate

# Repair checksums
flyway -url="jdbc:postgresql://localhost:5432/rust_clean_db" \
       -user=postgres -password=postgres \
       -locations=filesystem:./migrations repair
```

### SQLx Compile-Time Checks
```bash
# Generate offline query data (requires DATABASE_URL)
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rust_clean_db" \
  cargo sqlx prepare --workspace
```

## 3. Code Style & Conventions

### Formatting & Syntax
- Standard Rust formatting (`rustfmt`) is enforced.
- No unused imports. Remove them immediately.
- No `println!` macro in production code. Use `tracing::info!`, `warn!`, `error!`.
- No commented-out code.
- No non-ASCII characters in code or comments.
- No emojis in commit messages or code comments.
- **Strictly Forbidden**: Bypassing clippy lints using `#[allow(...)]` is prohibited. Fix the underlying issue instead.

### Naming Conventions
- Structs/Traits/Enums: `PascalCase`
- Variables/Functions/Modules: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Files: `snake_case.rs`. Use strict, descriptive names (e.g., `register_user_usecase.rs` instead of `register.rs` if ambiguous).

### Imports & Dependencies
- **Presentation layer** must use Application abstractions, NOT domain types directly.
- Use `Arc<dyn Trait>` for dependency injection.
- When creating new abstractions, add to `crates/application/src/abstractions/` and export in `mod.rs`.
- Implement abstractions in `crates/infrastructure/src/database/repository_impls/` or `crates/infrastructure/src/security/`.

### Error Handling
- Use `Result<T, E>` for all fallible operations.
- `crates/domain` defines core `DomainError`.
- `crates/application` defines `AppError` for presentation-layer errors.
- **Presentation handlers** should return `ApiError` (which wraps `AppError`).
- Convert domain errors to app errors in application layer: `impl From<DomainError> for AppError`.
- Do not use `unwrap()` or `expect()` in production code. Handle errors gracefully.
- Use `?` operator for cleaner propagation.

### Dependency Injection
- Use `std::sync::Arc` for shared ownership.
- Inject dependencies via constructor injection (functions like `new`).
- In `application`, depend on `Arc<dyn Trait>` (e.g., `Arc<dyn UserRepository>`), not concrete structs.

### Documentation
- Use `///` doc comments for all public structs, traits, and functions.
- Comments should be concise and explain *why* or *how* if complex.

## 4. Git & Version Control

### Commit Messages
- **Strictly follow Conventional Commits**.
- Use **lowercase** for the description.
- No emojis.
- Format: `<type>(<scope>): <description>`
- Types:
  - `feat`: New feature
  - `fix`: Bug fix
  - `refactor`: Code change that neither fixes a bug nor adds a feature
  - `chore`: Maintenance (deps, build, tools)
  - `docs`: Documentation changes
  - `test`: Adding or missing tests
- Example: `feat(auth): add login usecase`

### Branch Naming
- Format: `<type>/<kebab-case-description>`
- Examples:
  - `feature/add-user-registration`
  - `fix/login-error-handling`
  - `refactor/application-layer`

## 5. Implementation Guidelines

### General Rules
- **No Fake Implementations**: Do not write functions with `todo!()`, `unimplemented!()`, or empty bodies just to satisfy the compiler. You must implement the actual logic.
- **Test Driven**: Ideally, write tests for your use cases.

### Adding a New Feature
1. Define Entity/Value Object in `domain` (if needed).
2. Define Repository Interface in `domain`.
3. Create abstraction in `application/abstractions/` if infrastructure implementation needed.
4. Create Use Case in `application/usecases`.
5. Implement the Interface in `infrastructure`.
6. Expose via `presentation` (Handler/Controller).
7. Wire everything in `bins/*/main.rs`.

### Modifying Dependencies
- Always check `Cargo.toml` in the specific crate.
- Use `[workspace.dependencies]` in root `Cargo.toml` for version synchronization.
- Never add an outward dependency (e.g., infrastructure) to an inner layer (application/domain).

## 6. Agent Behavior
- Be concise.
- Verify your changes by running the Code Quality commands listed above.
- Do not make assumptions about existing code; use `grep` or `read` to confirm.
- If you encounter a violation of the Clean Architecture rules during your task, fix it or report it.
