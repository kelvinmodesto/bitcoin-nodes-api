# Bitcoin Nodes API

A Rust-based REST API that fetches Bitcoin Lightning Network node data from external sources, stores it in a PostgreSQL database, and exposes it through HTTP endpoints.

### Features

- ⚡ Fetches real-time Bitcoin Lightning Network node data from mempool.space API
- 🗄️ PostgreSQL database with connection pooling via Diesel ORM
- 🔄 Upsert operations for efficient data synchronization
- 🚀 High-performance async HTTP server using Actix Web
- 🛡️ Type-safe database operations with Rust's ownership system
- 📊 Structured logging and error handling

## Build tools & versions used

- **Rust**: 1.75+ (2021 edition)
- **PostgreSQL**: 12+
- **Diesel CLI**: 2.2+ with PostgreSQL support
- **Docker** (optional): For running PostgreSQL locally

### Dependencies

- **actix-web**: 4.11.0 - Web framework
- **diesel**: 2.2.12 - ORM and query builder
- **chrono**: 0.4.41 - Date and time handling
- **reqwest**: 0.12.22 - HTTP client
- **serde**: 1.0.219 - Serialization/deserialization
- **uuid**: 1.17.0 - UUID generation
- **tokio**: 1.46.1 - Async runtime

## Steps to run the app

### 1. Prerequisites

#### Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

#### Set Docker compose:
```bash
# Start PostgreSQL with Docker
docker-compose up -d

# The database will be available at postgresql://postgres:password@localhost:5432/bitcoin_nodes

```


#### Environment confiiguration
```bash
# Database Configuration
DATABASE_URL=postgresql://postgres:password123@localhost:5432/bitcoin_nodes
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password123
POSTGRES_DATABASE=bitcoin_nodes
POSTGRES_PORT=5432
POSTGRES_CONTAINER_NAME=bitcoin-nodes-postgres

# pgAdmin Configuration
PGADMIN_EMAIL=admin@admin.com
PGADMIN_PASSWORD=admin

# Server Configuration
SERVER_PORT=8080
RUST_LOG=debug

# Development flags
RUN_INTEGRATION_TESTS=1
```

#### Database Migrations
```bash
# Check migration status
diesel migration list

# Run pending migrations
diesel migration run

# Verify schema generation
diesel print-schema

```

#### Build and Run
```bash
# Build the project
cargo build --release

# Run the server
cargo run

# Or run in development mode with auto-reload
cargo watch -x run
```

This server will start at http://localhost:8080

#### Testing
```
# Run unit tests
cargo test

# Run integration tests (requires database)
RUN_INTEGRATION_TESTS=1 cargo test

```

## What was the reason for your focus? What problems were you trying to solve?
I was trying to create a simple API for managing Bitcoin nodes, but at the same time, I wanted to make it easy to use, understand and scale.

## How long did you spend on this project?
Between 4 and 6 hours. I'm not sure.

## Did you make any trade-offs for this project? What would you have done differently with more time?
Extra validations, I'd like to look more carefully my model and dto to avoid future issues, and more tests, specially e2e tests.

## What do you think is the weakest part of your project?
I didn't have enough time to implement all the features I wanted, per example, I didn't implement any cron routine that I planned to do running hourly a routine to upsert all nodes every hour without break data consistency. I lost some time testing mongodb and trying to make it work with the current implementation and I changed my mind and use a relational database instead(PostgreSQL).

## Is there any other information you’d like us to know?
Any AI assistance or vibe code was used to generate code, only regular research and development, it's far from perfect, but it's a good starting point with the deadline that I gave for myself.
