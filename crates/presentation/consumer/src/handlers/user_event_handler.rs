use tracing::{info, warn, error};
use std::clone::Clone;

use crate::models::UserEvent;

#[derive(Clone)]
pub struct UserEventHandler;

impl UserEventHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(&self, event: UserEvent) {
        info!("Received user event: type={}, user_id={}, email={}", 
            event.event_type, event.user_id, event.email);

        let result = match event.event_type.as_str() {
            "user.created" => {
                self.handle_user_created(&event).await
            }
            "user.updated" => {
                self.handle_user_updated(&event).await
            }
            "user.deleted" => {
                self.handle_user_deleted(&event).await
            }
            _ => {
                warn!("Unknown event type: {} for user: {}", event.event_type, event.user_id);
                Ok(())
            }
        };

        match result {
            Ok(_) => {
                info!("Successfully processed {} event for user: {}", event.event_type, event.user_id);
            }
            Err(e) => {
                error!("Failed to process {} event for user {}: {}", event.event_type, event.user_id, e);
            }
        }
    }

    async fn handle_user_created(&self, event: &UserEvent) -> Result<(), String> {
        info!("Creating user event log: email={}", event.email);
        
        // Could integrate with database here to log the event
        // For now, just log at info level
        info!("User created event processed for: {}", event.email);
        
        Ok(())
    }

    async fn handle_user_updated(&self, event: &UserEvent) -> Result<(), String> {
        info!("Updating user event log: email={}", event.email);
        
        // Could integrate with database here to log the event
        info!("User updated event processed for: {}", event.email);
        
        Ok(())
    }

    async fn handle_user_deleted(&self, event: &UserEvent) -> Result<(), String> {
        info!("Archiving user event log: email={}", event.email);
        
        // Could integrate with database here to archive/log the deletion
        info!("User deleted event processed for: {}", event.email);
        
        Ok(())
    }
}
