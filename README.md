# pg-rs

A layered PostgreSQL application framework for Rust, built on SeaORM and Axum.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  HTTP API (Axum + OpenAPI)
├──────────────────────────────────────────────────────┤
│                     demo-db                          │  Business API Layer
├──────────────────────────────────────────────────────┤
│                    pg-tables                         │  Domain Services & Entities
├──────────────────────────────────────────────────────┤
│                     pg-core                          │  Infrastructure (DB Manager)
├──────────────────────────────────────────────────────┤
│                    migration                         │  Schema Migrations
└──────────────────────────────────────────────────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `pg-core` | Database connection management, error handling, repository traits |
| `pg-tables` | SeaORM entities, domain services, DTOs |
| `demo-db` | Business API layer (HealthApi for medical data) |
| `web-server` | Axum HTTP server with OpenAPI/Swagger |
| `migration` | SeaORM database migrations |

## Data Model

Health observation system using star schema:

```
┌──────────┐     ┌─────────────┐     ┌────────────┐
│ subject  │────▶│ observation │◀────│   metric   │
└──────────┘     └─────────────┘     └────────────┘
                        │
                        ▼
                 ┌─────────────┐
                 │ data_source │
                 └─────────────┘
```

- **subject** - Entities being observed (person, device)
- **metric** - Measurable indicators (height, blood pressure, lab tests)
- **observation** - Recorded measurements (fact table)
- **data_source** - Origin of observations (manual, device, report)

## Quick Start

### 1. Prerequisites

- Rust 2024 Edition
- PostgreSQL 12+
- Docker (optional, for local PostgreSQL)

### 2. Setup Database

```bash
# Start PostgreSQL with Docker
make postgres

# Run migrations
make migrate-up

# Load sample data
psql -f sql/init_data.sql
```

### 3. Configure

Copy and edit configuration:

```bash
cp config/services-example.toml config/services.toml
```

Configuration options:

```toml
[http]
port = 19878

[[db]]
name = "default"
url = "postgres://user:password@localhost:5432/mydb"
max_connections = 10
min_connections = 1
connect_timeout = 30
idle_timeout = 600
sql_logging = false
```

### 4. Run Server

```bash
cargo run -p web-server
```

API available at:
- Swagger UI: http://localhost:19878/swagger-ui
- OpenAPI Spec: http://localhost:19878/api-docs/openapi.json

## API Endpoints

### Medical API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/medical/observations` | Query observations by subject and metric |

**Query Parameters:**
- `subject_id` - Subject ID (required)
- `metric_id` - Metric ID (required)
- `start_at` - Start time in RFC3339 format (optional)
- `end_at` - End time in RFC3339 format (optional)

## Usage Examples

### Database Connection

```rust
use pg_core::{DatabaseConfig, DatabaseManager};

let config = DatabaseConfig::new("main", "postgres://localhost/mydb")
    .max_connections(20)
    .with_sql_logging(true);

let manager = DatabaseManager::new(vec![config]).await?;
let db = manager.default()?;
```

### Multi-Database Setup

```rust
let configs = vec![
    DatabaseConfig::new("main", "postgres://localhost/main_db"),
    DatabaseConfig::new("replica", "postgres://replica/main_db"),
];

let manager = DatabaseManager::new(configs).await?;
let main_db = manager.get("main")?;
let replica_db = manager.get("replica")?;
```

### Using Services

```rust
use pg_tables::table::observation::ObservationService;
use pg_tables::table::observation::dto::RecordObservation;

let service = ObservationService::new(db);

// Record an observation
let input = RecordObservation {
    subject_id: 1,
    metric_id: 1,
    value: "175.5".to_string(),
    observed_at: OffsetDateTime::now_utc(),
    source_id: Some(1),
};
service.record(input).await?;
```

### Using HealthApi

```rust
use demo_db::api::medical::HealthApi;
use demo_db::dto::medical::QueryObservationSeries;

let api = HealthApi::new(db);

let query = QueryObservationSeries {
    subject_id: 1,
    metric_id: 1,
};
let range = Range { from: None, to: None };

let result = api.query_observation(query, range).await?;
```

## Development

### Make Commands

```bash
make help              # Show all commands

# Database
make postgres          # Start PostgreSQL with Docker
make migrate-up        # Run migrations
make migrate-down      # Rollback migrations
make migrate-fresh     # Drop all and re-run migrations
make migrate-gen NAME=xxx  # Generate new migration
make generate-entity   # Generate SeaORM entities

# Build & Test
make build             # Build all crates
make test              # Run tests
make clean             # Clean build artifacts
```

### Project Structure

```
pg-rs/
├── crates/
│   ├── pg-core/       # Infrastructure layer
│   ├── pg-tables/     # Domain layer
│   ├── demo-db/       # Business API layer
│   ├── web-server/    # HTTP API layer
│   └── migration/     # Database migrations
├── config/            # Configuration files
├── sql/               # SQL scripts
└── scripts/           # Development scripts
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Runtime | Tokio |
| ORM | SeaORM 2.0 |
| Web Framework | Axum 0.8 |
| OpenAPI | utoipa + Swagger UI |
| Serialization | serde |
| Validation | validator |
| Logging | tracing |
| Time | time 0.3 |

## License

MIT or Apache-2.0
