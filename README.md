# pg-rs

A layered PostgreSQL SDK for Rust built on SeaORM.

## Architecture

```
┌─────────────────┐
│   Your App      │
└────────┬────────┘
         │
         ├──────────────┬──────────────┐
         │              │              │
    ┌────▼─────┐   ┌───▼────┐   ┌────▼──────┐
    │ pg-core  │   │ pg-sdk │   │ migration │
    └──────────┘   └────────┘   └───────────┘
         │              │
         └──────┬───────┘
                │
         ┌──────▼──────┐
         │   SeaORM    │
         └─────────────┘
```

## Crates

### pg-core
**Database connection management layer**

- Multi-database connection management
- Connection pooling configuration
- Core error handling

```toml
[dependencies]
pg-core = { path = "path/to/pg-rs/crates/pg-core" }
```

### pg-sdk
**Business logic and data access layer**

- Entity definitions (SeaORM models)
- Business operations
- Domain-specific queries

```toml
[dependencies]
pg-sdk = { path = "path/to/pg-rs/crates/pg-sdk" }
```

### migration
**Database schema migrations**

- Schema version management
- Migration scripts
- Used via `sea-orm-cli`

```bash
sea-orm-cli migrate up -d crates/migration
sea-orm-cli migrate generate new_table -d crates/migration
```

## Quick Start

### 1. Add Dependencies

```toml
[dependencies]
pg-core = { path = "path/to/pg-rs/crates/pg-core" }
pg-sdk = { path = "path/to/pg-rs/crates/pg-sdk" }
tokio = { version = "1", features = ["full"] }
```

### 2. Setup Connection

```rust
use pg_core::{DatabaseConfig, DatabaseManager};

#[tokio::main]
async fn main() -> pg_core::Result<()> {
    let config = DatabaseConfig::new(
        "main",
        "postgres://user:password@localhost/mydb"
    )
    .max_connections(20)
    .with_sql_logging(true);

    let manager = DatabaseManager::new(vec![config]).await?;
    let db = manager.default()?;

    // Use db with pg-sdk business logic
    
    Ok(())
}
```

### 3. Multi-Database Setup

```rust
let configs = vec![
    DatabaseConfig::new("main", "postgres://localhost/main_db"),
    DatabaseConfig::new("replica", "postgres://replica/main_db"),
    DatabaseConfig::new("analytics", "postgres://localhost/analytics_db"),
];

let manager = DatabaseManager::new(configs).await?;

let main_db = manager.get("main")?;
let replica_db = manager.get("replica")?;
```

## Examples

The project includes runnable examples demonstrating how to use pg-core and pg-sdk:

```bash
# List available examples
cargo run --example

# Run basic usage example
cargo run --example basic_usage

# Run multi-database example
cargo run --example multi_database
```

See [examples/README.md](examples/README.md) for detailed setup instructions and prerequisites.

## Quick Commands

Use `make` commands from the project root:

```bash
# Show all available commands
make help

# Database operations
make fresh-db           # Refresh database and generate entities
make migrate-up         # Run migrations
make migrate-down       # Rollback migrations
make migrate-gen NAME=create_users  # Generate new migration

# Development
make build              # Build all crates
make test               # Run tests
make examples           # Build examples
make example-basic      # Run basic example
make example-multi      # Run multi-database example

# Utilities
make postgres           # Manage PostgreSQL with Docker
make init               # Initialize project
```

Or run scripts directly from `scripts/` directory - see [scripts/README.md](scripts/README.md).

## Development

### Running Migrations

```bash
# Generate new migration
sea-orm-cli migrate generate table_name -d crates/migration

# Run migrations
sea-orm-cli migrate up -d crates/migration

# Rollback
sea-orm-cli migrate down -d crates/migration
```

### Building

```bash
# Build all crates
cargo build

# Test
cargo test

# Build specific crate
cargo build -p pg-core
cargo build -p pg-sdk
```

## Design Philosophy

### Why Separate pg-core and pg-sdk?

1. **Separation of Concerns**
   - pg-core: Infrastructure (connections, pools)
   - pg-sdk: Business logic (entities, operations)

2. **Flexibility**
   - Apps can choose which business logic to include
   - Connection layer is reusable across projects

3. **Testing**
   - Business logic can be tested with mock connections
   - Connection layer tested independently

4. **Dependencies**
   - Not every app needs every business entity
   - Core connection management is always needed

## License

MIT or Apache-2.0
