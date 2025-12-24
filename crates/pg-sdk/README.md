# pg-sdk

Business logic and data access layer for PostgreSQL operations.

## Purpose

`pg-sdk` provides the business logic layer built on top of `pg-core`, including:

- Entity definitions (SeaORM models)
- Business logic and operations
- Data access patterns
- Query builders specific to your domain

## Usage

External crates should depend on both `pg-core` (for connection) and `pg-sdk` (for business logic).

```toml
[dependencies]
pg-core = { path = "path/to/pg-rs/crates/pg-core" }
pg-sdk = { path = "path/to/pg-rs/crates/pg-sdk" }
```

```rust
use pg_core::{DatabaseConfig, DatabaseManager};
use pg_sdk; // Your business logic

#[tokio::main]
async fn main() -> pg_core::Result<()> {
    // 1. Setup connection with pg-core
    let config = DatabaseConfig::new(
        "main",
        "postgres://user:password@localhost/mydb"
    );
    let manager = DatabaseManager::new(vec![config]).await?;
    let db = manager.default()?;

    // 2. Use pg-sdk business logic with the connection
    // let prompts = pg_sdk::prompt::list_all(db).await?;

    Ok(())
}
```

## Architecture

- **pg-core**: Connection management (imported by pg-sdk)
- **pg-sdk**: Business logic and entities (this crate)
- Your app: Uses both for complete functionality

## Why Two Crates?

- **Separation of Concerns**: Connection management vs business logic
- **Selective Dependencies**: Not all apps need all business logic
- **Cleaner Testing**: Can test business logic with mock connections
- **Better Organization**: Clear boundaries between layers
