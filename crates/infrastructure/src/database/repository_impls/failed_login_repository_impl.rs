use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use rust_clean_domain::{DomainError, FailedLogin, FailedLoginRepository};

use crate::database::FailedLoginModel;

pub struct PgFailedLoginRepository {
    pool: PgPool,
}

impl PgFailedLoginRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FailedLoginRepository for PgFailedLoginRepository {
    async fn create(&self, failed_login: &FailedLogin) -> Result<FailedLogin, DomainError> {
        let model = FailedLoginModel::from_domain(failed_login);

        let created = sqlx::query_as!(
            FailedLoginModel,
            r#"
            INSERT INTO failed_logins (
                id,
                email,
                ip_address,
                attempts,
                locked_until,
                last_attempt_at,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, email, ip_address, attempts, locked_until, last_attempt_at, created_at
            "#,
            model.id,
            model.email,
            model.ip_address,
            model.attempts,
            model.locked_until,
            model.last_attempt_at,
            model.created_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(created.to_domain())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<FailedLogin>, DomainError> {
        let model = sqlx::query_as!(
            FailedLoginModel,
            "SELECT id, email, ip_address, attempts, locked_until, last_attempt_at, created_at FROM failed_logins WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.map(|m| m.to_domain()))
    }

    async fn find_by_email_and_ip(
        &self,
        email: &str,
        ip_address: &str,
    ) -> Result<Option<FailedLogin>, DomainError> {
        let model = sqlx::query_as!(
            FailedLoginModel,
            "SELECT id, email, ip_address, attempts, locked_until, last_attempt_at, created_at FROM failed_logins WHERE email = $1 AND ip_address = $2",
            email,
            ip_address
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.map(|m| m.to_domain()))
    }

    async fn register_failed_attempt(
        &self,
        email: &str,
        ip_address: &str,
        max_attempts: i32,
        lockout_minutes: i64,
    ) -> Result<FailedLogin, DomainError> {
        let model = sqlx::query_as!(
            FailedLoginModel,
            r#"
            INSERT INTO failed_logins (
                id,
                email,
                ip_address,
                attempts,
                locked_until,
                last_attempt_at,
                created_at
            )
            VALUES ($1, $2, $3, 1, NULL, NOW(), NOW())
            ON CONFLICT ON CONSTRAINT failed_logins_email_ip_unique
            DO UPDATE SET
                attempts = failed_logins.attempts + 1,
                last_attempt_at = NOW(),
                locked_until = CASE
                    WHEN failed_logins.attempts + 1 >= $4 THEN NOW() + make_interval(mins => $5::int)
                    ELSE failed_logins.locked_until
                END
            RETURNING id, email, ip_address, attempts, locked_until, last_attempt_at, created_at
            "#,
            Uuid::new_v4(),
            email,
            ip_address,
            max_attempts,
            lockout_minutes as i32,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(model.to_domain())
    }

    async fn update(&self, failed_login: &FailedLogin) -> Result<FailedLogin, DomainError> {
        let model = FailedLoginModel::from_domain(failed_login);

        let updated = sqlx::query_as!(
            FailedLoginModel,
            r#"
            UPDATE failed_logins
            SET attempts = $2, locked_until = $3, last_attempt_at = $4
            WHERE email = $1
            RETURNING id, email, ip_address, attempts, locked_until, last_attempt_at, created_at
            "#,
            model.email,
            model.attempts,
            model.locked_until,
            model.last_attempt_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(updated.to_domain())
    }

    async fn delete(&self, email: &str) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM failed_logins WHERE email = $1", email)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM failed_logins WHERE locked_until < NOW() OR (locked_until IS NULL AND last_attempt_at < NOW() - INTERVAL '24 hours')"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
