# Scripts

Utility scripts for database management and development.

## Recommended: Use Makefile

For convenience, you can use `make` commands from the project root:

```bash
make help        # Show all available commands
make fresh-db    # Run fresh_db.sh
make postgres    # Run postgres.sh
make init        # Run init.sh
```

See the main [README.md](../README.md) for all available make commands.

## Direct Script Usage

You can also run scripts directly:

## Available Scripts

### fresh_db.sh

Complete database refresh and entity generation workflow.

**What it does:**
1. Runs `migrate refresh` - drops all tables and re-runs all migrations
2. Generates SeaORM entities from the current database schema
3. Outputs entities to `crates/pg-sdk/src/entity`

**Usage:**
```bash
# From project root
./scripts/fresh_db.sh

# Or from scripts directory
cd scripts && ./fresh_db.sh
```

**Requirements:**
- `sea-orm-cli` installed
- PostgreSQL running
- `DATABASE_URL` environment variable set

### postgres.sh

PostgreSQL Docker management script.

**Usage:**
```bash
./scripts/postgres.sh
```

### init.sh

Project initialization script.

**Usage:**
```bash
./scripts/init.sh
```

## Environment Setup

Make sure you have `DATABASE_URL` set:

```bash
export DATABASE_URL="postgres://user:password@localhost:5432/dbname"
```

Or create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost:5432/dbname
```

## Common Workflows

### Starting Fresh

```bash
# 1. Start PostgreSQL
./scripts/postgres.sh

# 2. Run migrations and generate entities
./scripts/fresh_db.sh
```

### After Adding New Migration

```bash
# Just run fresh_db.sh to apply and regenerate entities
./scripts/fresh_db.sh
```
