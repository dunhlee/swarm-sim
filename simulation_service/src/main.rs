use common::dispatcher::RpcDispatcher;
use common::network::run_server;
use common::commands::command::Command;
use common::message_layer::messaging;
use common::message_types::types::SimulationUpdate;
use serde_json::{json, Value};
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


// pub struct AddCommand;
//
// #[async_trait::async_trait]
// impl Command for AddCommand
// {
//     fn name(&self) -> &'static str {
//         "add"
//     }
//
//     async fn execute(&self, params: Value) -> Value {
//         let a = params["a"].as_f64().unwrap();
//         let b = params["b"].as_f64().unwrap();
//         json!(a + b)
//     }
// }

