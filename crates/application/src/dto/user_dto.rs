use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use rust_clean_domain::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<&rust_clean_domain::User> for UserResponse {
    fn from(user: &rust_clean_domain::User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            role: user.role.to_string(),
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

impl ErrorResponse {
    pub fn new(error: String, message: String, status_code: u16) -> Self {
        Self {
            error,
            message,
            status_code,
        }
    }
}
