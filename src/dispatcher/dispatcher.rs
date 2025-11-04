use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::protocol::{RpcRequest, RpcResponse};

// Each function handler takes in json parameters and returns json parameters
pub type FunctionHandler = Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output = Value> + Send>> + Send + Sync>;

#[derive(Clone)]
pub struct RpcDispatcher {
    pub handlers: Arc<RwLock<HashMap<String, FunctionHandler>>>,
}

impl RpcDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register<F, Fut>(&self, name: &str, handler: F)
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Value> + Send + 'static,
    {
        // acquire lock
        let mut handlers = self.handlers.write().await;
        handlers.insert(name.to_string(), Arc::new(move |params| {
            Box::pin(handler(params))
        }));
    }

    pub async fn dispatch(&self, request: RpcRequest) -> RpcResponse {
        let handlers = self.handlers.read().await;
        let response: RpcResponse;
        if let Some(handler) = handlers.get(&request.method) {
            let result = handler(request.params.clone()).await;
            response = RpcResponse::success(request.id, result)
        }
        else {
            response = RpcResponse::error(request.id, -1, "Method not found");
        }

        response
    }
}
