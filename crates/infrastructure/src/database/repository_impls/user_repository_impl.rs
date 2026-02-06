use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use rust_clean_domain::{DomainError, User, UserRepository};

fn convert_role_from_str(role_str: &str) -> rust_clean_domain::UserRole {
    match role_str {
        "Admin" => rust_clean_domain::UserRole::Admin,
        _ => rust_clean_domain::UserRole::User,
    }
}

fn convert_role_to_string(role: &rust_clean_domain::UserRole) -> String {
    match role {
        rust_clean_domain::UserRole::Admin => "Admin".to_string(),
        rust_clean_domain::UserRole::User => "User".to_string(),
    }
}

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
        let row: Option<(Uuid, String, String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        match row {
            Some((id, email, password_hash, name, role, created_at, updated_at)) => {
                let domain_role = convert_role_from_str(&role);
                Ok(Some(User {
                    id,
                    email,
                    password_hash,
                    name,
                    role: domain_role,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let row: Option<(Uuid, String, String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        match row {
            Some((id, email, password_hash, name, role, created_at, updated_at)) => {
                let domain_role = convert_role_from_str(&role);
                Ok(Some(User {
                    id,
                    email,
                    password_hash,
                    name,
                    role: domain_role,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        let rows: Vec<(Uuid, String, String, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, email, password_hash, name, role, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let users: Vec<User> = rows
            .into_iter()
            .map(
                |(id, email, password_hash, name, role, created_at, updated_at)| {
                    let domain_role = convert_role_from_str(&role);
                    User {
                        id,
                        email,
                        password_hash,
                        name,
                        role: domain_role,
                        created_at,
                        updated_at,
                    }
                },
            )
            .collect();
        Ok(users)
    }

    async fn create(&self, user: &User) -> Result<User, DomainError> {
        let row: (
            Uuid,
            String,
            String,
            String,
            String,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
        ) = sqlx::query_as(
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

        let domain_role = convert_role_from_str(&row.4);
        Ok(User {
            id: row.0,
            email: row.1,
            password_hash: row.2,
            name: row.3,
            role: domain_role,
            created_at: row.5,
            updated_at: row.6,
        })
    }

    async fn update(&self, user: &User) -> Result<User, DomainError> {
        let row: (
            Uuid,
            String,
            String,
            String,
            String,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
        ) = sqlx::query_as(
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
        .bind(convert_role_to_string(&user.role))
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let domain_role = convert_role_from_str(&row.4);
        Ok(User {
            id: row.0,
            email: row.1,
            password_hash: row.2,
            name: row.3,
            role: domain_role,
            created_at: row.5,
            updated_at: row.6,
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
