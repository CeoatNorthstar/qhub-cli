use anyhow::{Context, Result};
use sqlx::{Pool, Postgres, Sqlite};
use std::env;

/// Database pool that supports both PostgreSQL (local) and SQLite (Cloudflare D1)
#[derive(Clone)]
pub enum DatabasePool {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

impl DatabasePool {
    /// Create a new database pool from environment configuration
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL must be set")?;

        if database_url.starts_with("postgres") {
            Self::new_postgres(&database_url).await
        } else if database_url.starts_with("sqlite") || database_url.starts_with("file:") {
            Self::new_sqlite(&database_url).await
        } else {
            anyhow::bail!("Unsupported database URL format. Use 'postgres://' or 'sqlite://'")
        }
    }

    /// Create a PostgreSQL pool (for local development)
    async fn new_postgres(url: &str) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .context("Failed to connect to PostgreSQL")?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run PostgreSQL migrations")?;

        Ok(DatabasePool::Postgres(pool))
    }

    /// Create a SQLite pool (for Cloudflare D1 compatibility)
    async fn new_sqlite(url: &str) -> Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .context("Failed to connect to SQLite")?;

        // Run migrations (use D1-compatible schema)
        // Note: In production with D1, migrations are handled via wrangler
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run SQLite migrations")?;

        Ok(DatabasePool::Sqlite(pool))
    }

    /// Get the underlying postgres pool
    pub fn postgres(&self) -> Result<&Pool<Postgres>> {
        match self {
            DatabasePool::Postgres(pool) => Ok(pool),
            DatabasePool::Sqlite(_) => {
                anyhow::bail!("Expected PostgreSQL pool, but got SQLite")
            }
        }
    }

    /// Get the underlying sqlite pool
    pub fn sqlite(&self) -> Result<&Pool<Sqlite>> {
        match self {
            DatabasePool::Sqlite(pool) => Ok(pool),
            DatabasePool::Postgres(_) => {
                anyhow::bail!("Expected SQLite pool, but got PostgreSQL")
            }
        }
    }

    /// Check if using PostgreSQL
    pub fn is_postgres(&self) -> bool {
        matches!(self, DatabasePool::Postgres(_))
    }

    /// Check if using SQLite
    pub fn is_sqlite(&self) -> bool {
        matches!(self, DatabasePool::Sqlite(_))
    }
}
