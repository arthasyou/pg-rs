# pg-core

PostgreSQL connection management layer built on SeaORM.

## Purpose

`pg-core` provides the database connection layer, including:
- Multi-database connection management
- Connection pooling configuration
- Error handling for database operations

## Usage

This crate is typically used together with `pg-tables` which provides business logic.

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

    // Pass db to your business logic layer (pg-tables)

    Ok(())
}
```

## Features

- **DatabaseConfig**: Flexible database configuration with builder pattern
- **DatabaseManager**: Manages multiple named database connections
- **Connection Pooling**: Built-in connection pool management via SeaORM
- **Error Handling**: Comprehensive error types for database operations

## Configuration Options

```rust
DatabaseConfig::new("name", "connection_url")
    .max_connections(20)         // Max connections in pool (default: 10)
    .min_connections(5)          // Min connections in pool (default: 1)
    .connect_timeout(60)         // Connection timeout in seconds (default: 30)
    .idle_timeout(600)           // Idle timeout in seconds (default: 600)
    .with_sql_logging(true);     // Enable SQL logging (default: false)
```
