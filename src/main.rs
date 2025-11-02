use anyhow::Result;

mod network;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Swarm Sim Server");
    network::run_server("127.0.0.1:8080").await?;
    Ok(())
}
