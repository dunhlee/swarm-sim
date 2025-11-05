use serde::{Deserialize, Serialize};
use serde_json::Value;

// client request to server
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub method: String,
    #[serde(default)]
    pub params: Value, // Value's default value is null. serde default will default to null if there are no parameters.
}

// server response to client
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>
}

impl RpcResponse {
    pub fn success(id: Option<u64>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None
        }
    }

    pub fn error(id: Option<u64>, error_code: RpcErrorCode) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(RpcError::from(error_code)),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RpcErrorCode {
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    ServerUnavailable = -32604,
    Timeout = -32605,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl RpcError {
    pub fn new(error_code: i32, message: &str, data: Option<Value>) -> Self {
        Self {
            code: error_code,
            message: message.into(),
            data
        }
    }
}

impl From<RpcErrorCode> for RpcError {
    fn from(code: RpcErrorCode) -> Self {
        RpcError::new(code as i32, code.to_string().as_str(), None)
    }
}

impl std::fmt::Display for RpcErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RpcErrorCode::ParseError => "Parse Error",
            RpcErrorCode::InvalidRequest => "Invalid Request",
            RpcErrorCode::MethodNotFound => "Method Not Found",
            RpcErrorCode::InvalidParams => "Invalid Params",
            RpcErrorCode::InternalError => "Internal Error",
            RpcErrorCode::ServerUnavailable => "Server Unavailable",
            RpcErrorCode::Timeout => "Timeout",
        })
    }
}

