mod dispatcher;
mod protocol;
mod network;
mod commands;

use dispatcher::RpcDispatcher;
use serde_json::{json, Value};
use network::run_server;
use crate::commands::command::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dispatcher = RpcDispatcher::new();
    dispatcher.register(AddCommand).await;

    run_server("127.0.0.1:8080", dispatcher).await?;
    Ok(())
}


pub struct AddCommand;

#[async_trait::async_trait]
impl Command for AddCommand
{
    fn name(&self) -> &'static str {
        "add"
    }

    async fn execute(&self, params: Value) -> Value {
        let a = params["a"].as_f64().unwrap();
        let b = params["b"].as_f64().unwrap();
        json!(a + b)
    }
}
