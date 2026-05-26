#![allow(clippy::must_use_candidate, clippy::missing_errors_doc)]
//! JSON-RPC protocol types and helpers for the MCP server.

use serde::Deserialize;
use serde_json::Value;
use std::io::{self, Write};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    #[serde(default, deserialize_with = "deserialize_request_id")]
    pub id: RequestId,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum RequestId {
    #[default]
    Absent,
    Null,
    Value(Value),
}

impl RequestId {
    pub fn into_response(self) -> Option<Value> {
        match self {
            RequestId::Absent => None,
            RequestId::Null => Some(Value::Null),
            RequestId::Value(v) => Some(v),
        }
    }
}

pub fn deserialize_request_id<'de, D>(d: D) -> Result<RequestId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match Option::<Value>::deserialize(d) {
        Ok(Some(v)) => Ok(RequestId::Value(v)),
        Ok(None) => Ok(RequestId::Null),
        Err(e) => Err(e),
    }
}

#[derive(serde::Serialize)]
pub struct JsonRpcResponse {
    jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(serde::Serialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

pub fn rpc_error(id: RequestId, code: i32, message: String) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id: id.into_response(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message,
            data: None,
        }),
    }
}

pub fn rpc_result(id: RequestId, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id: id.into_response(),
        result: Some(result),
        error: None,
    }
}

pub fn rpc_no_response() -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id: None,
        result: None,
        error: None,
    }
}

pub fn write_response(resp: &JsonRpcResponse) {
    if let Ok(line) = serde_json::to_string(resp) {
        let mut stdout = io::stdout().lock();
        let _ = writeln!(stdout, "{line}");
        let _ = stdout.flush();
    }
}
