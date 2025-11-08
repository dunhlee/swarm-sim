use anyhow::Result;
use async_nats::{Client, ConnectOptions};

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