use lapin::{Connection, ConnectionProperties};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RabbitMQError {
    #[error("Failed to connect to RabbitMQ: {0}")]
    ConnectionError(String),
}

pub struct RabbitMQConnection {
    pub connection: Connection,
}

impl RabbitMQConnection {
    pub async fn new(url: &str) -> Result<Self, RabbitMQError> {
        let connection = Connection::connect(url, ConnectionProperties::default())
            .await
            .map_err(|e| RabbitMQError::ConnectionError(e.to_string()))?;

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
}
