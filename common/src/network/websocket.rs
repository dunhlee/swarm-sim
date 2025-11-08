use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use anyhow::Result;

use crate::protocol::{RpcResponse, RpcRequest};
use crate::dispatcher::RpcDispatcher;
use crate::protocol::protocol::RpcErrorCode;

pub async fn start_websocket_server(socket_addr: &str, rpc_dispatcher: RpcDispatcher) -> Result<()>
{
    let listener = TcpListener::bind(socket_addr).await?;
    println!("Listening on: {}", socket_addr);

    while let Ok((stream, _)) = listener.accept().await {
        let dispatcher = rpc_dispatcher.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, dispatcher).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, rpc_dispatcher: RpcDispatcher) -> Result<()>  {
    let websocket_stream = accept_async(stream).await?;
    println!("WebSocket connection established! Client Connected.");

    // splits websocket into a sink and a stream
    let (mut write, mut read) = websocket_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                // deserialize json from stream into an RpcRequest Struct
                if let Ok(req) = serde_json::from_str::<RpcRequest>(&text) {

                    // dispatch the request to the handler
                    let response = rpc_dispatcher.dispatch(req).await;

                    // serialize the response as json and send it back to the client
                    let json = serde_json::to_string(&response)?;
                    write.send(Message::Text(json)).await?;

                } else {
                    let error = RpcResponse::error(Some(0), RpcErrorCode::InvalidRequest.into());
                    let json = serde_json::to_string(&error)?;
                    write.send(Message::Text(json)).await?;
                }
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}