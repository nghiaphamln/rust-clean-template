use async_trait::async_trait;
use rust_clean_application::abstractions::BruteForceProtection;
use rust_clean_application::error_types::AppError;
use rust_clean_domain::FailedLoginRepository;

use crate::database::PgFailedLoginRepository;

pub struct PgBruteForceProtection {
    repo: PgFailedLoginRepository,
}

impl PgBruteForceProtection {
    pub fn new(repo: PgFailedLoginRepository) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl BruteForceProtection for PgBruteForceProtection {
    async fn is_locked(&self, email: &str, ip_address: &str) -> Result<bool, AppError> {
        let result = self
            .repo
            .find_by_email_and_ip(email, ip_address)
            .await
            .map_err(AppError::from)?;

        Ok(result.is_some_and(|f| f.is_locked()))
    }

    async fn record_failure(
        &self,
        email: &str,
        ip_address: &str,
        max_attempts: i32,
        lockout_minutes: i64,
    ) -> Result<(), AppError> {
        self.repo
            .register_failed_attempt(email, ip_address, max_attempts, lockout_minutes)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    async fn clear_failures(&self, email: &str) -> Result<(), AppError> {
        self.repo.delete(email).await.map_err(AppError::from)?;
        Ok(())
    }
}
