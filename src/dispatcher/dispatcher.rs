use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use crate::protocol::{RpcRequest, RpcResponse};
use crate::protocol::protocol::RpcErrorCode;
use crate::commands::command::Command;

// Each function handler takes in json parameters and returns json parameters
pub type CommandMap = Arc<RwLock<HashMap<String, Arc<dyn Command>>>>;

#[derive(Clone)]
pub struct RpcDispatcher {
    commands: CommandMap,
}

impl RpcDispatcher {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register<C>(&self, command: C)
    where
        C: Command + 'static,
    {
        // acquire lock
        let mut command_map = self.commands.write().await;
        command_map.insert(command.name().to_string(), Arc::new(command));
    }

    pub async fn dispatch(&self, request: RpcRequest) -> RpcResponse {

        if request.is_valid()
        {
            let command_map = self.commands.read().await;

            return if let Some(command) = command_map.get(&request.method) {
                let result = command.execute(request.params).await;
                RpcResponse::success(request.id, result)
            } else {
                RpcResponse::error(request.id, RpcErrorCode::MethodNotFound.into())
            }
        }

        RpcResponse::error(request.id, RpcErrorCode::InvalidRequest.into())
    }
}
