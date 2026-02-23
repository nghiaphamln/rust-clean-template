use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use rust_clean_domain::{DomainError, RefreshToken, RefreshTokenRepository};

use crate::database::RefreshTokenModel;

pub struct PgRefreshTokenRepository {
    pool: PgPool,
}

impl PgRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn create(&self, token: &RefreshToken) -> Result<RefreshToken, DomainError> {
        let model = RefreshTokenModel::from_domain(token);
        let created: RefreshTokenModel = sqlx::query_as(
            r#"
            INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, token_hash, expires_at, created_at
            "#,
        )
        .bind(model.id)
        .bind(model.user_id)
        .bind(model.token_hash)
        .bind(model.expires_at)
        .bind(model.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(created.to_domain())
    }

    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<RefreshToken>, DomainError> {
        let model: Option<RefreshTokenModel> = sqlx::query_as(
            "SELECT id, user_id, token_hash, expires_at, created_at FROM refresh_tokens WHERE token_hash = $1"
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.map(|m| m.to_domain()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
