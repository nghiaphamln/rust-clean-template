use dotenvy::dotenv;
use std::env;
use tracing::{info, level_filters::LevelFilter};

use rust_clean_consumer_lib::{models::UserEvent, UserEventHandler};
use rust_clean_infrastructure::{
    rabbitmq::{RabbitMQConnection, RabbitMQConsumer},
    Database,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_max = env::var("DATABASE_POOL_MAX_SIZE")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .unwrap_or(5);

    let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let queue_name = env::var("RABBITMQ_QUEUE").unwrap_or_else(|_| "rust_clean_queue".to_string());

    info!("Connecting to database...");
    let _database = Database::new(&database_url, pool_max)
        .await
        .expect("Failed to connect to database");
    info!("Database connected");

    info!("Connecting to RabbitMQ...");
    let rabbitmq_connection = RabbitMQConnection::new(&rabbitmq_url)
        .await
        .expect("Failed to connect to RabbitMQ");
    info!("RabbitMQ connected");

    let consumer = RabbitMQConsumer::new(rabbitmq_connection, queue_name);
    let handler = UserEventHandler::new();

    info!("Starting consumer...");
    consumer
        .start(move |data| {
            let handler = handler.clone();
            async move {
                if let Ok(event) = UserEvent::from_json(&data) {
                    handler.handle(event).await;
                } else {
                    tracing::error!("Failed to parse message as UserEvent");
                }
            }
        })
        .await
        .expect("Failed to start consumer");

    Ok(())
}
