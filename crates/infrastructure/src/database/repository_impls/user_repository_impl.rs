use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use rust_clean_domain::{DomainError, User, UserRepository};

use crate::database::UserModel;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let model: Option<UserModel> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let model: Option<UserModel> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let models: Vec<UserModel> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }

    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let model: UserModel = sqlx::query_as(
            r#"
            INSERT INTO users (id, email, password_hash, name, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, email, password_hash, name, role, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(user.role.to_string())
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.to_domain())
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let model: UserModel = sqlx::query_as(
            r#"
            UPDATE users
            SET email = $2, password_hash = $3, name = $4, role = $5, updated_at = $6
            WHERE id = $1
            RETURNING id, email, password_hash, name, role, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.name)
        .bind(user.role.to_string())
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
