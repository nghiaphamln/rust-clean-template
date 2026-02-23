use async_trait::async_trait;

use crate::{DomainError, FailedLogin};

#[async_trait]
pub trait FailedLoginRepository: Send + Sync {
    async fn create(&self, failed_login: &FailedLogin) -> Result<FailedLogin, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<FailedLogin>, DomainError>;
    async fn find_by_email_and_ip(
        &self,
        email: &str,
        ip_address: &str,
    ) -> Result<Option<FailedLogin>, DomainError>;
    async fn register_failed_attempt(
        &self,
        email: &str,
        ip_address: &str,
        max_attempts: i32,
        lockout_minutes: i64,
    ) -> Result<FailedLogin, DomainError>;
    async fn update(&self, failed_login: &FailedLogin) -> Result<FailedLogin, DomainError>;
    async fn delete(&self, email: &str) -> Result<(), DomainError>;
    async fn delete_expired(&self) -> Result<(), DomainError>;
}
