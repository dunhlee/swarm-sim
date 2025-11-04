mod dispatcher;
mod protocol;
mod network;

use dispatcher::RpcDispatcher;
use serde_json::json;
use network::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dispatcher = RpcDispatcher::new();
    dispatcher.register("ping", handle_ping).await;

    run_server("127.0.0.1:8080", dispatcher).await?;
    Ok(())
}

async fn handle_ping(_params: serde_json::Value) -> serde_json::Value {
    json!("pong")
}
