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
    pub fn from_json(data: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(data)
    }
}
