use common::dispatcher::RpcDispatcher;
use common::network::run_server;
use common::commands::command::Command;
use common::message_layer::messaging;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let new_client = messaging::connect_nats("nats://127.0.0.1:4222").await?;

    loop
    {
        let telemetry = json!({
            "agent_id": 1,
            "pos": [12.3, 7.8],
            "status": "OK",
        });

        let payload = serde_json::to_vec(&telemetry)?;
        new_client.publish("Telemetry.updates:", payload.into()).await?;

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

