.PHONY: help fresh-db postgres init build test examples clean

# Default target
help:
	@echo "Available commands:"
	@echo "  make fresh-db    - Refresh database and generate entities"
	@echo "  make postgres    - Manage PostgreSQL with Docker"
	@echo "  make init        - Initialize project"
	@echo ""
	@echo "  make build       - Build all crates"
	@echo "  make test        - Run all tests"
	@echo "  make examples    - Build all examples"
	@echo "  make clean       - Clean build artifacts"
	@echo ""
	@echo "  make migrate-up     - Run migrations"
	@echo "  make migrate-down   - Rollback migrations"
	@echo "  make migrate-fresh  - Fresh migrations (drop all & rerun)"

# Scripts
fresh-db:
	@./scripts/fresh_db.sh

postgres:
	@./scripts/postgres.sh

init:
	@./scripts/init.sh

# Cargo commands
build:
	cargo build

test:
	cargo test

examples:
	cargo build --examples

clean:
	cargo clean

# Migration commands
migrate-up:
	sea-orm-cli migrate up -d crates/migration

migrate-down:
	sea-orm-cli migrate down -d crates/migration

migrate-fresh:
	sea-orm-cli migrate fresh -d crates/migration

migrate-refresh:
	sea-orm-cli migrate refresh -d crates/migration

# Generate new migration
# Usage: make migrate-gen NAME=create_users
migrate-gen:
	@if [ -z "$(NAME)" ]; then \
		echo "Error: NAME is required. Usage: make migrate-gen NAME=create_users"; \
		exit 1; \
	fi
	sea-orm-cli migrate generate $(NAME) -d crates/migration

# Generate entities
generate-entity:
	sea-orm-cli generate entity -o crates/pg-sdk/src/entity --with-serde both

# Run examples
example-basic:
	cargo run --example basic_usage

example-multi:
	cargo run --example multi_database
