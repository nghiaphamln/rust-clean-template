use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use rust_clean_domain::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserModel {
    pub fn to_domain(&self) -> rust_clean_domain::User {
        rust_clean_domain::User {
            id: self.id,
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            name: self.name.clone(),
            role: UserRole::from(self.role.as_str()),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn from_domain(user: &rust_clean_domain::User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            name: user.name.clone(),
            role: user.role.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
