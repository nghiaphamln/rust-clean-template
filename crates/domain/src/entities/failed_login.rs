use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailedLogin {
    pub id: Uuid,
    pub email: String,
    pub ip_address: String,
    pub attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_attempt_at: DateTime<Utc>,
}

impl FailedLogin {
    pub fn new(email: String, ip_address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            ip_address,
            attempts: 1,
            locked_until: None,
            last_attempt_at: Utc::now(),
        }
    }

    pub fn increment_attempt(&mut self) {
        self.attempts += 1;
        self.last_attempt_at = Utc::now();
    }

    pub fn lock(&mut self, duration_minutes: i64) {
        self.locked_until = Some(Utc::now() + chrono::Duration::minutes(duration_minutes));
    }

    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            return Utc::now() < locked_until;
        }
        false
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
        self.locked_until = None;
        self.last_attempt_at = Utc::now();
    }
}
