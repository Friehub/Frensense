// SPDX-License-Identifier: MIT
//! MCP request dispatcher.

use super::audit::{run_audit, run_audit_streamed, tool_definition};
use super::protocol::{JsonRpcRequest, JsonRpcResponse, rpc_error, rpc_no_response, rpc_result};
use serde_json::{Value, json};

pub fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => {
            let result = json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "frensense-mcp",
                    "version": crate::FRENSENSE_VERSION
                }
            });
            rpc_result(req.id, result)
        }

        "notifications/initialized" | "notifications/cancelled" => rpc_no_response(),

        "shutdown" => rpc_result(req.id, json!(null)),

        "ping" => rpc_result(req.id, json!("pong")),

        "tools/list" => {
            let result = json!({
                "tools": [tool_definition()]
            });
            rpc_result(req.id, result)
        }

        "tools/call" => {
            let name = req.params.get("name").and_then(Value::as_str).unwrap_or("");
            if name != "frensense_audit" {
                return rpc_error(req.id, -32602, format!("unknown tool: {name}"));
            }

            let args = &req.params["arguments"];

            let path = args
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or(".")
                .to_string();

            let fix_auto = args
                .get("fix_auto")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            let severity_threshold = args
                .get("severity_threshold")
                .and_then(Value::as_str)
                .unwrap_or("warning")
                .to_string();

            let stream = args.get("stream").and_then(Value::as_bool).unwrap_or(false);

            let language = args.get("language").and_then(Value::as_str);

            let rules: Option<Vec<String>> =
                args.get("rules").and_then(Value::as_array).map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                });

            let rules_slice = rules.as_deref();

            if stream {
                run_audit_streamed(
                    req.id,
                    &path,
                    fix_auto,
                    &severity_threshold,
                    language,
                    rules_slice,
                );
                return rpc_no_response();
            }

            let result_data =
                run_audit(&path, fix_auto, &severity_threshold, language, rules_slice);

            let result = json!({
                "content": [
                    {
                        "type": "text",
                        "text": serde_json::to_string_pretty(&result_data).unwrap_or_default()
                    }
                ]
            });

            rpc_result(req.id, result)
        }

        _ => rpc_error(req.id, -32601, format!("method not found: {}", req.method)),
    }
}
