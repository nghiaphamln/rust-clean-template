use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),

    #[error("Migration failed: {0}")]
    MigrationError(String),
}

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
