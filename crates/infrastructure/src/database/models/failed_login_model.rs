use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FailedLoginModel {
    pub id: Uuid,
    pub email: String,
    pub ip_address: String,
    pub attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_attempt_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl FailedLoginModel {
    pub fn to_domain(&self) -> rust_clean_domain::FailedLogin {
        rust_clean_domain::FailedLogin {
            id: self.id,
            email: self.email.clone(),
            ip_address: self.ip_address.clone(),
            attempts: self.attempts,
            locked_until: self.locked_until,
            last_attempt_at: self.last_attempt_at,
        }
    }

    pub fn from_domain(failed_login: &rust_clean_domain::FailedLogin) -> Self {
        Self {
            id: failed_login.id,
            email: failed_login.email.clone(),
            ip_address: failed_login.ip_address.clone(),
            attempts: failed_login.attempts,
            locked_until: failed_login.locked_until,
            last_attempt_at: failed_login.last_attempt_at,
            created_at: failed_login.last_attempt_at,
        }
    }
}
