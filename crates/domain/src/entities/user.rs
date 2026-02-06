use crate::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
        }
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String, name: String) -> Result<Self, DomainError> {
        if email.is_empty() {
            return Err(DomainError::ValidationError(
                "Email cannot be empty".to_string(),
            ));
        }
        if !email.contains('@') {
            return Err(DomainError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }
        if name.is_empty() {
            return Err(DomainError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }
        if password_hash.len() < 60 {
            return Err(DomainError::ValidationError(
                "Invalid password hash".to_string(),
            ));
        }

        Ok(User {
            id: Uuid::new_v4(),
            email,
            password_hash,
            name,
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn with_role(mut self, role: UserRole) -> Self {
        self.role = role;
        self.updated_at = Utc::now();
        self
    }
}
