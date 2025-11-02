pub mod websocket;

use anyhow::Result ;
use crate::network::websocket::start_websocket_server;

pub async fn run_server(addr: &str) -> Result<()> {
    start_websocket_server(addr).await?;
    Ok(())
}