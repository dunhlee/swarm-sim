use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use anyhow::Result;
use serde_json::json;
use crate::protocol::{RpcResponse, RpcRequest, RpcError};

pub async fn start_websocket_server(socket_addr: &str) -> Result<()>
{
    let listener = TcpListener::bind(socket_addr).await?;
    println!("Listening on: {}", socket_addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()>  {
    let websocket_stream = accept_async(stream).await?;
    println!("WebSocket connection established! Client Connected.");

    // splits websocket into a sink and a stream
    let (mut write, mut read) = websocket_stream.split();

    while let Some(message) = read.next().await {
        let message = message?;

        if message.is_text() || message.is_binary() {
            let text = message.into_text()?;

            match serde_json::from_str::<RpcRequest>(&text) {
                Ok(request) => {
                    let id = request.id;
                    let response = match request.method.as_str() {
                        "ping" => RpcResponse::success(id, json!("pong")),
                        other => RpcResponse::error(id, -3, &format!("Unknown method: {}", other)),
                    };

                    let json = serde_json::to_string(&response)?;
                    write.send(Message::Text(json)).await?;
                }
                Err(e) => {
                    eprintln!("Failed to parse RPC message: {}", e);
                    let response = RpcResponse::error(None, -3, "Invalid JSON");
                    let json = serde_json::to_string(&response)?;
                    write.send(Message::Text(json)).await?;
                }
            }
        }
    }
    
    Ok(())
}