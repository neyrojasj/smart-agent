---
name: rust-web-app
description: Scaffold a production-ready Rust web application with Axum, Tokio, SQLx, and PostgreSQL.
version: "1.0"
---

# Rust Web App Skill

## Identity

- **Name**: rust-web-app
- **Version**: 1.0
- **Description**: Creates a fully scaffolded Rust web application with Axum, Tokio, SQLx, PostgreSQL, Makefile, Docker Compose, and agent memory initialization.

---

## Triggers

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: rust web app, axum, scaffold rust | "Create a new Rust web app" |
| "new rust api" | "Scaffold a Rust REST API" |
| "setup rust project" | "Setup a new Rust web project" |

---

## Capabilities

- ✅ Scaffold full Rust web project (Axum + Tokio + SQLx + PostgreSQL)
- ✅ Create layered source structure (routes, handlers, models, db, middleware)
- ✅ Generate Cargo.toml with production-ready dependencies
- ✅ Generate Makefile with dev/test/db/docker commands
- ✅ Generate docker-compose.yml with PostgreSQL
- ✅ Generate .env.example and .gitignore
- ✅ Apply Rust standards from `.github/copilot/standards/rust.md`
- ✅ Initialize agent memory via the setup skill

---

## Dependencies

- `.github/copilot/standards/rust.md` — Rust coding standards (applied in Step 10)
- Chains to: `setup` skill (Step 11 — agent memory initialization)

---

## Required Inputs

Before starting, collect from the user:

| Input | Description | Example |
|-------|-------------|---------|
| **Project Name** | snake_case name | `my_api` |
| **Description** | Brief description | `REST API for user management` |

---

## Workflow

### Step 1: Initialize Cargo Project

```bash
cargo new [PROJECT_NAME] --name [PROJECT_NAME]
cd [PROJECT_NAME]
```

---

### Step 2: Create Project Structure

```
[PROJECT_NAME]/
├── Cargo.toml
├── Makefile
├── README.md
├── .env.example
├── .gitignore
├── migrations/
│   └── .gitkeep
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   └── health.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── health.rs
│   ├── models/
│   │   └── mod.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── pool.rs
│   └── middleware/
│       └── mod.rs
└── tests/
    └── integration/
        ├── mod.rs
        └── health_test.rs
```

---

### Step 3: Create Cargo.toml

```toml
[package]
name = "[PROJECT_NAME]"
version = "0.1.0"
edition = "2021"
authors = ["[AUTHOR_NAME]"]
description = "[DESCRIPTION]"
license = "MIT"
readme = "README.md"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace", "timeout"] }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error Handling
thiserror = "2"
anyhow = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Configuration
dotenvy = "0.15"

# Validation
validator = { version = "0.18", features = ["derive"] }

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
reqwest = { version = "0.12", features = ["json"] }

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

### Step 4: Create Core Source Files

#### src/main.rs
```rust
use [PROJECT_NAME]::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
```

#### src/lib.rs
```rust
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;
    let pool = db::pool::create_pool(&config.database_url).await?;

    let app = Router::new()
        .merge(routes::routes(pool))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

#### src/config.rs
```rust
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: std::env::var("HOST").context("HOST must be set")?,
            port: std::env::var("PORT")
                .context("PORT must be set")?
                .parse()
                .context("PORT must be a valid number")?,
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            rust_log: std::env::var("RUST_LOG").context("RUST_LOG must be set")?,
        })
    }
}
```

#### src/error.rs
```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Database(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
        };

        let body = Json(json!({ "error": message, "status": status.as_u16() }));
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
```

#### src/routes/mod.rs
```rust
pub mod health;

use axum::Router;
use sqlx::PgPool;

pub fn routes(pool: PgPool) -> Router {
    Router::new()
        .merge(health::routes())
        .with_state(pool)
}
```

#### src/routes/health.rs
```rust
use axum::{routing::get, Router};
use crate::handlers;

pub fn routes() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/health/ready", get(handlers::health::readiness_check))
}
```

#### src/handlers/health.rs
```rust
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn health_check() -> Json<Value> {
    Json(json!({ "status": "healthy", "version": env!("CARGO_PKG_VERSION") }))
}

pub async fn readiness_check(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, StatusCode> {
    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(json!({ "status": "ready", "database": "connected" })))
}
```

#### src/db/pool.rs
```rust
use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .context("Failed to connect to database")
}
```

---

### Step 5: Create .env.example

```env
HOST=127.0.0.1
PORT=3000
DATABASE_URL=postgres://postgres:password@localhost:5432/[PROJECT_NAME]
RUST_LOG=info,[PROJECT_NAME]=debug
```

---

### Step 6: Create Makefile

```makefile
.PHONY: help build run dev test lint fmt clean docker-up docker-down migrate

PROJECT_NAME := [PROJECT_NAME]
DOCKER_COMPOSE := docker compose

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

build: ## Build release
	cargo build --release

run: ## Run the application
	cargo run

dev: ## Run with hot-reload (requires cargo-watch)
	cargo watch -x run

test: ## Run all tests
	cargo test

lint: ## Run clippy
	cargo clippy -- -D warnings

fmt: ## Format code
	cargo fmt

audit: ## Audit dependencies
	cargo audit

migrate: ## Run database migrations
	sqlx migrate run

migrate-create: ## Create migration (NAME=migration_name)
	sqlx migrate add $(NAME)

docker-up: ## Start Docker services
	$(DOCKER_COMPOSE) up -d

docker-down: ## Stop Docker services
	$(DOCKER_COMPOSE) down

clean: ## Clean build artifacts
	cargo clean

install-tools: ## Install development tools
	cargo install cargo-watch cargo-tarpaulin cargo-audit sqlx-cli

setup: install-tools docker-up ## Full project setup
```

---

### Step 7: Create docker-compose.yml

```yaml
services:
  postgres:
    image: postgres:15-alpine
    container_name: [PROJECT_NAME]_db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: [PROJECT_NAME]
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
```

---

### Step 8: Create .gitignore

```gitignore
/target/
.env
.env.local
.env.*.local
.idea/
.vscode/
*.swp
*.swo
.DS_Store
*.log
tarpaulin-report.html
coverage/
```

---

### Step 9: Create README.md

Use the project name and description to generate a README with:
- Tech stack table (Axum, Tokio, SQLx, PostgreSQL)
- Quick Start section (prerequisites, setup steps)
- Available `make` commands table
- API Endpoints table (starting with `/health`, `/health/ready`)
- Project structure diagram

---

### Step 10: Apply Rust Standards

Read `.github/copilot/standards/rust.md` if it exists. Enforce:
- Use `thiserror` for library errors, `anyhow` for application errors
- No `.unwrap()` without a documented reason — use `.context()` instead
- No default values for env vars — fail fast if missing
- All public items must be documented

---

### Step 11: Initialize Agent Memory

Chain to the **setup** skill to initialize `.github/copilot/docs/` documentation for the new project.

---

### Step 12: Report Summary

```
✅ **Rust Web App Initialized**

**Project**: [PROJECT_NAME]
**Stack**: Axum + SQLx + PostgreSQL

**Files created**:
- Cargo.toml
- Makefile
- docker-compose.yml
- README.md
- src/ (application code)
- .env.example

**Next steps**:
1. `cp .env.example .env` and configure values
2. `make docker-up` to start PostgreSQL
3. `make migrate` to run migrations
4. `make dev` to start development server

Health check available at: http://localhost:3000/health
```

---

## Never Do

- ❌ Use `.unwrap()` without justification
- ❌ Provide default fallbacks for environment variables
- ❌ Skip running the setup skill after scaffolding
- ❌ Leave `[PROJECT_NAME]` or `[DESCRIPTION]` placeholders unfilled
