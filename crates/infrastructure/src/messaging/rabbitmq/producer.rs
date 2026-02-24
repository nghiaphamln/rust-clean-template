use lapin::{options::*, types::FieldTable, BasicProperties, ExchangeKind};
use thiserror::Error;
use tracing::info;

use crate::rabbitmq::RabbitMQConnection;

#[derive(Debug, Error)]
pub enum ProducerError {
    #[error("Failed to declare exchange: {0}")]
    ExchangeDeclareError(String),

    #[error("Failed to publish message: {0}")]
    PublishError(String),
}

pub struct RabbitMQProducer {
    connection: RabbitMQConnection,
    exchange_name: String,
}

impl RabbitMQProducer {
    pub fn new(connection: RabbitMQConnection, exchange_name: String) -> Self {
        Self {
            connection,
            exchange_name,
        }
    }

    pub async fn publish(&self, routing_key: &str, message: &[u8]) -> Result<(), ProducerError> {
        let conn = self.connection.get_connection();
        let channel = conn
            .create_channel()
            .await
            .map_err(|e| ProducerError::PublishError(e.to_string()))?;

        channel
            .exchange_declare(
                self.exchange_name.as_str().into(),
                ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ProducerError::ExchangeDeclareError(e.to_string()))?;

        let _confirm = channel
            .basic_publish(
                self.exchange_name.as_str().into(),
                routing_key.into(),
                BasicPublishOptions::default(),
                message,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| ProducerError::PublishError(e.to_string()))?;

        info!(
            "Message published to exchange '{}' with routing key '{}'",
            self.exchange_name, routing_key
        );

        Ok(())
    }

    pub async fn publish_json<T: serde::Serialize>(
        &self,
        routing_key: &str,
        message: &T,
    ) -> Result<(), ProducerError> {
        let json =
            serde_json::to_vec(message).map_err(|e| ProducerError::PublishError(e.to_string()))?;
        self.publish(routing_key, &json).await
    }
}
