use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use anyhow::Result;

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
            println!("Received message: {:?}", message.to_text()?);
            write.send(message).await?;
        }
    }

    println!("Client disconnected.");
    Ok(())
}