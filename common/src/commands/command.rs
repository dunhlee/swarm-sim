use serde_json::value::Value;

// defines the Command Trait
use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
    /// Which NATS subject does this command respond to?
    fn subject(&self) -> &'static str;

    /// Execute the command using a raw JSON payload.
    async fn execute(&self, payload: &[u8]);
}
