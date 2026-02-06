use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    pub event_type: String,
    pub user_id: Uuid,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

impl UserEvent {
    pub fn new(event_type: String, user_id: Uuid, email: String) -> Self {
        Self {
            event_type,
            user_id,
            email,
            timestamp: Utc::now(),
        }
    }

    pub fn user_created(user_id: Uuid, email: String) -> Self {
        Self::new("user.created".to_string(), user_id, email)
    }

    pub fn user_updated(user_id: Uuid, email: String) -> Self {
        Self::new("user.updated".to_string(), user_id, email)
    }

    pub fn user_deleted(user_id: Uuid, email: String) -> Self {
        Self::new("user.deleted".to_string(), user_id, email)
    }
}
