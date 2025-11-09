use common::message_layer::messaging;
use common::message_layer::messaging::subscribe_json;
use common::message_types::types::SimulationUpdate;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let new_client = messaging::connect_nats("nats://127.0.0.1:4222").await?;

    println!("Telemetry subscriber listening on simulation-updates");

    subscribe_json::<SimulationUpdate, _>(&new_client, "simulation-updates", |data| {
        println!("Received message: {:?}", data);
    }).await?;

    futures::future::pending::<()>().await;
    Ok(())
}