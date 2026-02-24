use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable};
use thiserror::Error;
use tracing::info;

use crate::rabbitmq::RabbitMQConnection;

#[derive(Debug, Error)]
pub enum ConsumerError {
    #[error("Failed to declare queue: {0}")]
    QueueDeclareError(String),

    #[error("Failed to start consuming: {0}")]
    ConsumeError(String),
}

pub struct RabbitMQConsumer {
    connection: RabbitMQConnection,
    queue_name: String,
}

impl RabbitMQConsumer {
    pub fn new(connection: RabbitMQConnection, queue_name: String) -> Self {
        Self {
            connection,
            queue_name,
        }
    }

    pub async fn start<F, Fut>(&self, handler: F) -> Result<(), ConsumerError>
    where
        F: Fn(Vec<u8>) -> Fut + Clone + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let conn = self.connection.get_connection();
        let channel = conn
            .create_channel()
            .await
            .map_err(|e| ConsumerError::ConsumeError(e.to_string()))?;

        let queue = channel
            .queue_declare(
                self.queue_name.as_str().into(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::QueueDeclareError(e.to_string()))?;

        let queue_name_str = queue.name();

        info!("Queue '{}' declared", queue_name_str);

        let mut consumer = channel
            .basic_consume(
                queue_name_str.as_str().into(),
                "rust-clean-consumer".into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| ConsumerError::ConsumeError(e.to_string()))?;

        info!("Started consuming from queue '{}'", queue_name_str);

        while let Some(delivery) = consumer.next().await {
            let delivery = match delivery {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("Error in consumer: {}", e);
                    continue;
                }
            };

            let data = delivery.data.to_vec();
            let handler = handler.clone();

            tokio::spawn(async move {
                handler(data).await;
            });

            if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                tracing::error!("Failed to ack message: {}", e);
            }
        }

        Ok(())
    }
}
