# Examples

This directory contains examples demonstrating how to use pg-core and pg-tables.

## Prerequisites

1. **PostgreSQL Running**
   ```bash
   # Using Docker
   docker run -d \
     --name postgres-example \
     -e POSTGRES_PASSWORD=postgres \
     -e POSTGRES_USER=postgres \
     -p 5432:5432 \
     postgres:latest
   ```

2. **Create Test Databases**
   ```bash
   # Connect to PostgreSQL
   psql -h localhost -U postgres
   
   # Create databases
   CREATE DATABASE test_db;
   CREATE DATABASE main_db;
   CREATE DATABASE replica_db;
   CREATE DATABASE analytics_db;
   ```

## Running Examples

### Basic Usage

Demonstrates basic connection and usage of pg-core and pg-tables:

```bash
# Using default connection
cargo run --example basic_usage

# Using custom DATABASE_URL
DATABASE_URL="postgres://user:pass@localhost/mydb" cargo run --example basic_usage
```

### Multi-Database

Demonstrates managing multiple database connections:

```bash
# Using default connections
cargo run --example multi_database

# Using custom database URLs
MAIN_DATABASE_URL="postgres://user:pass@localhost/main" \
REPLICA_DATABASE_URL="postgres://user:pass@replica-host/main" \
ANALYTICS_DATABASE_URL="postgres://user:pass@localhost/analytics" \
cargo run --example multi_database
```

## Example Structure

Each example demonstrates:

1. **Configuration** - Setting up `DatabaseConfig`
2. **Connection** - Creating `DatabaseManager`
3. **Usage** - Accessing and using database connections
4. **Error Handling** - Handling common errors

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/test_db` | Database connection URL for basic example |
| `MAIN_DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/main_db` | Main database URL |
| `REPLICA_DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/replica_db` | Replica database URL |
| `ANALYTICS_DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/analytics_db` | Analytics database URL |

## Notes

- Examples will fail gracefully if databases don't exist
- SQL logging can be observed in the console output when enabled
- Check the connection output to debug any connection issues
