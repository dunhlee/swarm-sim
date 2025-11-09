use anyhow::Result;
use async_nats::{Client, ConnectOptions};
use futures::StreamExt;
use serde::{Serialize, de::DeserializeOwned};

pub async fn publish_json<T: Serialize> (client: &Client, subject: &str, value: &T) -> Result<()>
{
    let payload = serde_json::to_vec(value)?;
    let subj = subject.to_string();

    client.publish(subj, payload.into()).await?;
    Ok(())
}

pub async fn subscribe_json<T, F> (client: &Client, subject: &str, handler: F) -> Result<()>
where
    T: DeserializeOwned + Send + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    let subj = subject.to_string();
    let mut sub = client.subscribe(subj.clone()).await?;

    tokio::spawn(async move {
        while let Some(msg) = sub.next().await {
            if let Ok(value) = serde_json::from_slice::<T>(&msg.payload) {
                handler(value);
            } else {
                eprintln!("Failed to deserialize message on: {:?}", sub);
            }
        }
    });

    Ok(())
}

// url format: "nats://127.0.0.1:4222"
// creates a client that connects to NATS broker
pub async fn connect_nats(url: &str) -> Result<Client>
{
    let client = ConnectOptions::new()
        .name("swarm-sim-client")
        .connect(url)
        .await?;

    println!("Messaging layer: connected to NATS at {}", url);
    Ok(client)
}