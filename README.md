# pg-rs

A layered PostgreSQL application framework for Rust, designed for **AI-driven development**.

> **Design Philosophy**: This project is built for AI (Claude Code / Cursor / Codex) to write business code. Humans only need to describe requirements.

## Key Features

- **AI-Friendly Architecture**: 5 clearly separated layers, easy for AI to understand and follow
- **Protocol-Driven**: `ai_protocols/` contains execution protocols ensuring code quality
- **Ready-to-Use Templates**: `how_to_use_ai.md` provides copy-paste prompt templates
- **Zero Hand-Written Code**: Ideally, humans only describe requirements, AI does all the coding

## Quick Start (AI Development Mode)

### 1. Setup Environment

```bash
# Start PostgreSQL
make postgres

# Run migrations
make migrate-up
```

### 2. Let AI Develop Features

Open Claude Code / Cursor / Codex and paste:

```
Please read ai_protocols/TABLE_ADDING_PROTOCOL.md first to understand the project architecture.

Then help me implement:
<describe your requirements>
```

Or use templates from `how_to_use_ai.md`.

### 3. Verify

```bash
cargo check
cargo run -p web-server
```

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  Layer 5: HTTP API
├──────────────────────────────────────────────────────┤
│                     demo-db                          │  Layer 4: Business API
├──────────────────────────────────────────────────────┤
│                    pg-tables                         │  Layer 3: Domain Services
├──────────────────────────────────────────────────────┤
│                     pg-core                          │  Layer 2: Infrastructure (frozen)
├──────────────────────────────────────────────────────┤
│                    migration                         │  Layer 1: Schema
└──────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Crate | Responsibility | AI Modifiable |
|-------|-------|----------------|---------------|
| 5 | web-server | HTTP routes, handlers, request/response DTOs | Yes |
| 4 | demo-db | Business APIs, orchestrate multiple services | Yes |
| 3 | pg-tables | Single-table services, DTOs, entities | Yes |
| 2 | pg-core | Connection pool, error handling, repository traits | **No** |
| 1 | migration | Database schema | Yes |

## AI Development Documentation

| File | Purpose |
|------|---------|
| `ai_protocols/TABLE_ADDING_PROTOCOL.md` | AI execution protocol (must read) |
| `how_to_use_ai.md` | Prompt templates (copy & paste) |
| `README_CN.md` | Chinese documentation |

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

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/medical/observations` | Query observations by subject and metric |
| POST | `/medical/observations` | Record observation with source |

Swagger UI: http://localhost:19878/swagger-ui

## Example: Let AI Add a New Feature

Copy to Claude Code:

```
Please follow ai_protocols/TABLE_ADDING_PROTOCOL.md strictly.

Implement complete feature:

### 1. New Table
- Table: user
- Fields:
  - user_id: bigint (PK, auto-increment)
  - username: varchar(255) (unique)
  - email: varchar(255)
  - created_at: timestamp

### 2. Business API
- Feature: User registration and query
- Input: username, email
- Output: user info

### 3. HTTP Endpoints
- POST /users - Create user
- GET /users/{id} - Get user

Follow 5-layer architecture:
1. migration → create table
2. Prompt me to run script for entity generation
3. pg-tables → dto + service
4. demo-db → api + dto
5. web-server → handler + route + dto

Do NOT modify pg-core.
```

## Development Commands

```bash
make help              # Show all commands
make postgres          # Start PostgreSQL
make migrate-up        # Run migrations
make migrate-fresh     # Rebuild database
make build             # Build project
make test              # Run tests
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Runtime | Tokio |
| ORM | SeaORM 2.0 |
| Web Framework | Axum 0.8 |
| OpenAPI | utoipa + Swagger UI |
| Serialization | serde |
| Logging | tracing |

## Why AI-Driven Development?

1. **Consistency**: AI strictly follows protocols, unified code style
2. **Efficiency**: Describing requirements is 10x faster than writing code
3. **Quality**: Protocol constraints prevent common mistakes
4. **Maintainability**: Clear layered architecture, easy for newcomers (including AI)

## License

MIT or Apache-2.0
