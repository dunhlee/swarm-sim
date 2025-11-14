use common::message_layer::messaging;
use common::message_types::types::SimulationUpdate;
use tokio::time::{sleep, Duration};
use anyhow::Result;
use common::message_layer::messaging::publish_json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = messaging::connect_nats("nats://127.0.0.1:4222").await?;
    println!("Telemetry publisher listening on simulation-updates");

    let update_message: SimulationUpdate = SimulationUpdate
    {
        id: 1,
        tick: 1,
        position: [1.0, 3.0, 4.0],
    };

    loop
    {
        publish_json(&client, "simulation-updates", &update_message).await?;

        println!("Published Update");
        sleep(Duration::from_secs(1)).await;
    }
}

