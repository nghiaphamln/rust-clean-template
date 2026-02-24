use async_trait::async_trait;

use crate::error_types::AppError;

#[async_trait]
pub trait BruteForceProtection: Send + Sync {
    async fn is_locked(&self, email: &str, ip_address: &str) -> Result<bool, AppError>;

    async fn record_failure(
        &self,
        email: &str,
        ip_address: &str,
        max_attempts: i32,
        lockout_minutes: i64,
    ) -> Result<(), AppError>;

    async fn clear_failures(&self, email: &str) -> Result<(), AppError>;
}
