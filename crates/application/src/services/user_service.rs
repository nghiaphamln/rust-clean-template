use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use rust_clean_domain::{User, UserRepository, DomainError};
use crate::dto::UserResponse;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User not found")]
    NotFound,
}

pub struct UserService<T: UserRepository> {
    repository: Arc<T>,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>, DomainError> {
        let users = self.repository.find_all().await?;
        Ok(users.iter().map(UserResponse::from).collect())
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<UserResponse, DomainError> {
        let user = self.repository.find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;
        Ok(UserResponse::from(&user))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn update_user(&self, id: Uuid, name: Option<String>) -> Result<UserResponse, DomainError> {
        let mut user = self.repository.find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".to_string()))?;
        
        if let Some(new_name) = name {
            user.name = new_name;
        }
        
        let updated_user = self.repository.update(&user).await?;
        Ok(UserResponse::from(&updated_user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_clean_domain::{User, UserRole};
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_user_response_from_user() {
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

        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.name, "Test User");
        assert!(response.role.contains("user"));
    }
}
