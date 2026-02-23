.PHONY: setup test lint doc run-api run-consumer migrate clean

setup: docker-up migrate build

build:
	cargo build --all

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down -v

migrate:
	@if command -v flyway >/dev/null 2>&1; then \
		flyway -url="jdbc:postgresql://localhost:5432/rust_clean_db" \
			   -user=postgres -password=postgres \
			   -locations=filesystem:./migrations migrate; \
	else \
		echo "Flyway not found. Please install Flyway CLI."; \
	fi

run-api:
	cargo run --bin api

run-consumer:
	cargo run --bin consumer

test:
	cargo test --all -- --test-threads=1

lint:
	cargo fmt --all -- --check
	cargo clippy --all -- -D warnings

doc:
	cargo doc --document-private-items --no-deps

clean:
	cargo clean
	docker-compose down -v
