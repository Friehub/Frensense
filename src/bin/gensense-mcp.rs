// SPDX-License-Identifier: MIT
//! GenSense MCP Server — stdin/stdout JSON-RPC bridge to the GenSense engine.
//!
//! Implements the Model Context Protocol so AI agents (Claude Code, etc.) can
//! use GenSense as a first-class semantic analysis tool.

use gensense::{Advisory, Engine, Severity};
use serde::Deserialize;
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use std::path::Path;

// ── JSON-RPC types ───────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    #[serde(default, deserialize_with = "deserialize_request_id")]
    id: RequestId,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Clone, PartialEq)]
enum RequestId {
    Absent,
    Null,
    Value(Value),
}

impl RequestId {
    fn into_response(self) -> Option<Value> {
        match self {
            RequestId::Absent => None,
            RequestId::Null => Some(Value::Null),
            RequestId::Value(v) => Some(v),
        }
    }
}

impl Default for RequestId {
    fn default() -> Self {
        RequestId::Absent
    }
}

fn deserialize_request_id<'de, D>(d: D) -> Result<RequestId, D::Error>
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
struct JsonRpcResponse {
    jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(serde::Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

fn rpc_error(id: RequestId, code: i32, message: String) -> JsonRpcResponse {
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

fn rpc_result(id: RequestId, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id: id.into_response(),
        result: Some(result),
        error: None,
    }
}

fn rpc_no_response() -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0",
        id: None,
        result: None,
        error: None,
    }
}

fn write_response(resp: &JsonRpcResponse) {
    if let Ok(line) = serde_json::to_string(resp) {
        let mut stdout = io::stdout().lock();
        let _ = writeln!(stdout, "{line}");
        let _ = stdout.flush();
    }
}

// ── Tool implementation ──────────────────────────────────────────────────────

fn tool_definition() -> Value {
    json!({
        "name": "gensense_audit",
        "description": "Run semantic analysis on a file or directory. Returns advisories the agent must resolve before code is considered correct. An empty advisories array and clean=true means the code satisfies all invariants.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File or directory path to audit"
                },
                "fix_auto": {
                    "type": "boolean",
                    "default": false,
                    "description": "Apply auto-fixable remediations in-place"
                },
                "severity_threshold": {
                    "type": "string",
                    "enum": ["critical", "warning", "info"],
                    "default": "warning",
                    "description": "Minimum severity to report (critical=only critical, warning=critical+warning, info=all)"
                }
            },
            "required": ["path"]
        }
    })
}

fn run_audit(path: &str, fix_auto: bool, severity_threshold: &str) -> Value {
    let target = Path::new(path);
    if !target.exists() {
        return json!({
            "clean": false,
            "advisories": [],
            "auto_fixed": 0,
            "requires_human": [],
            "error": format!("path does not exist: {}", path)
        });
    }

    // Map severity_threshold string to our Severity enum.
    // "critical" -> report only Critical
    // "warning"  -> report Critical + Warning
    // "info"     -> report everything
    let threshold = match severity_threshold {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    };

    let mut engine = Engine::new();
    let rule_count = engine.list_rules().len();
    eprintln!(
        "gensense-mcp: cwd={:?}, rules={}, threshold={:?}",
        std::env::current_dir().ok(),
        rule_count,
        severity_threshold
    );

    let advisories = match engine.run(target) {
        Ok(advisories) => advisories,
        Err(e) => {
            return json!({
                "clean": false,
                "advisories": [],
                "auto_fixed": 0,
                "requires_human": [],
                "error": format!("analysis error: {}", e)
            });
        }
    };

    // Filter by severity threshold
    let filtered: Vec<Advisory> = advisories
        .into_iter()
        .filter(|a| severity_rank(a.severity) >= severity_rank(threshold))
        .collect();

    let auto_fixable_count = filtered
        .iter()
        .filter(|a| a.proposed_replacement.is_some())
        .count() as u64;

    let requires_human: Vec<Advisory> = filtered
        .iter()
        .filter(|a| a.requires_human || a.proposed_replacement.is_none())
        .cloned()
        .collect();

    #[cfg(feature = "remediation")]
    if fix_auto {
        let _ = apply_auto_fixes(&filtered, target);
    }

    json!({
        "clean": filtered.is_empty(),
        "advisories": serde_json::to_value(&filtered).unwrap_or_default(),
        "auto_fixed": auto_fixable_count,
        "requires_human": serde_json::to_value(&requires_human).unwrap_or_default()
    })
}

fn severity_rank(s: Severity) -> u8 {
    match s {
        Severity::Critical => 3,
        Severity::Warning => 2,
        Severity::Info => 1,
    }
}

#[cfg(feature = "remediation")]
fn apply_auto_fixes(
    advisories: &[Advisory],
    root: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use gensense::patcher::PatchManager;

    let project_root = find_project_root_for_fix(root);
    let patcher = PatchManager::new(&project_root);

    let mut fixable: Vec<&Advisory> = advisories
        .iter()
        .filter(|a| a.proposed_replacement.is_some())
        .collect();

    // Sort DESC by start_byte to avoid offset drift
    fixable.sort_by_key(|a| std::cmp::Reverse(a.start_byte));

    for adv in &fixable {
        let _ = patcher.apply_fix(adv, Path::new(&adv.file_path));
    }

    Ok(())
}

#[cfg(feature = "remediation")]
fn find_project_root_for_fix(target: &Path) -> std::path::PathBuf {
    let mut root = target.to_path_buf();
    if root.is_file() {
        root = root.parent().unwrap_or(&root).to_path_buf();
    }
    while root.parent().is_some() {
        if root.join(".gensense").exists() || root.join(".git").exists() {
            break;
        }
        root = root.parent().expect("parent").to_path_buf();
    }
    root
}

// ── MCP dispatcher ───────────────────────────────────────────────────────────

fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => {
            let result = json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "gensense-mcp",
                    "version": gensense::GENSENSE_VERSION
                }
            });
            rpc_result(req.id, result)
        }

        "notifications/initialized" => rpc_no_response(),
        "notifications/cancelled" => rpc_no_response(),

        "shutdown" => rpc_result(req.id, json!(null)),

        "tools/list" => {
            let result = json!({
                "tools": [tool_definition()]
            });
            rpc_result(req.id, result)
        }

        "tools/call" => {
            let name = req.params.get("name").and_then(Value::as_str).unwrap_or("");
            if name != "gensense_audit" {
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

            let result_data = run_audit(&path, fix_auto, &severity_threshold);

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

// ── Main loop ────────────────────────────────────────────────────────────────

fn main() {
    eprintln!("gensense-mcp v{} starting", gensense::GENSENSE_VERSION);
    eprintln!(
        "gensense-mcp: cwd={:?}, has_rust={}",
        std::env::current_dir().ok(),
        cfg!(feature = "rust")
    );

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("gensense-mcp: stdin read error: {e}");
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = rpc_error(RequestId::Absent, -32700, format!("parse error: {e}"));
                write_response(&err_resp);
                continue;
            }
        };

        // Shutdown and exit — honour the MCP lifecycle
        if req.method == "exit" {
            break;
        }

        let resp = handle_request(req);
        if resp.id.is_some() {
            write_response(&resp);
        }
    }

    eprintln!("gensense-mcp: exiting");
}
