use common::message_layer::messaging;
use serde_json::Value;
use futures::StreamExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let new_client = messaging::connect_nats("nats://127.0.0.1:4222").await?;
    let mut sub = new_client.subscribe("Telemetry.updates:").await?;

    println!("Telemetry subscriber listening on telemetry updates");

    while let Some(message) = sub.next().await {
        let data: Value = serde_json::from_slice(&message.payload)?;
        println!("Received message: {}", data);
    }

    Ok(())
}