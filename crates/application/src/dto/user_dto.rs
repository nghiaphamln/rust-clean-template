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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_clean_domain::{User, UserRole};
    use uuid::Uuid;

    #[test]
    fn test_user_response_from_impl() {
        let now = Utc::now();
        let user = User {
            id: Uuid::nil(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            name: "Test User".to_string(),
            role: UserRole::User,
            created_at: now,
            updated_at: now,
        };

        let response = UserResponse::from(&user);

        assert_eq!(response.id, Uuid::nil());
        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.name, "Test User");
        assert!(response.role.contains("user"));
    }

    #[test]
    fn test_error_response_new() {
        let response = ErrorResponse::new(
            "test_error".to_string(),
            "This is a test error".to_string(),
            400,
        );

        assert_eq!(response.error, "test_error");
        assert_eq!(response.message, "This is a test error");
        assert_eq!(response.status_code, 400);
    }

    #[test]
    fn test_user_update_request() {
        let request = UserUpdateRequest {
            name: Some("New Name".to_string()),
        };

        assert_eq!(request.name, Some("New Name".to_string()));
    }
}
