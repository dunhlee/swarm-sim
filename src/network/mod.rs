pub mod websocket;
use anyhow::Result ;
use crate::dispatcher::RpcDispatcher;
use crate::network::websocket::start_websocket_server;

pub async fn run_server(addr: &str, request: RpcDispatcher) -> Result<()> {
    start_websocket_server(addr, request).await?;
    Ok(())
}