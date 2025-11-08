use serde_json::value::Value;

// defines the Command Trait
#[async_trait::async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    async fn execute(&self, params: Value) -> Value;
}