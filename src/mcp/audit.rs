// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

#![allow(clippy::print_stderr)]
//! Audit orchestration logic for the MCP server.

use super::protocol::{RequestId, rpc_result, write_response};
use crate::{Advisory, Engine, Severity};
use serde_json::{Value, json};
use std::io::{self, Write};
use std::path::Path;

pub fn tool_definition() -> Value {
    json!({
        "name": "frensense_audit",
        "description": "Run deterministic dataflow analysis on a file or directory. Returns advisories the agent must resolve before code is considered correct. An empty advisories array and clean=true means the code satisfies all invariants.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File or directory path to audit"
                },
                "severity_threshold": {
                    "type": "string",
                    "enum": ["critical", "warning", "info"],
                    "default": "warning",
                    "description": "Minimum severity to report"
                },
                "language": {
                    "type": "string",
                    "description": "Filter by language (rust, typescript, javascript, python)"
                }
            },
            "required": ["path"]
        }
    })
}

pub fn write_notification(params: &Value) {
    let notification = json!({
        "jsonrpc": "2.0",
        "method": "notification",
        "params": params
    });
    if let Ok(line) = serde_json::to_string(&notification) {
        let mut stdout = io::stdout().lock();
        let _ = writeln!(stdout, "{line}");
        let _ = stdout.flush();
    }
}

pub fn filter_advisories(
    advisories: Vec<Advisory>,
    severity_threshold: &str,
    language: Option<&str>,
) -> Vec<Advisory> {
    let threshold = match severity_threshold {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    };

    let extensions = language.and_then(crate::parser::extensions_for);

    advisories
        .into_iter()
        .filter(|a| severity_rank(a.severity) >= severity_rank(threshold))
        .filter(|a| {
            if let Some(exts) = extensions {
                let ext = Path::new(&a.file_path)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                exts.contains(&ext)
            } else {
                true
            }
        })
        .collect()
}

pub fn run_audit_streamed(
    id: RequestId,
    path: &str,
    severity_threshold: &str,
    language: Option<&str>,
) {
    let target = Path::new(path);
    if !target.exists() {
        let result = json!({
            "clean": false,
            "advisories": [],
            "error": format!("path does not exist: {}", path)
        });
        write_response(&rpc_result(id, result));
        return;
    }

    let mut engine = Engine::new();

    let advisories = match engine.run(target) {
        Ok(a) => a,
        Err(e) => {
            let result = json!({
                "clean": false,
                "advisories": [],
                "error": format!("analysis error: {}", e)
            });
            write_response(&rpc_result(id, result));
            return;
        }
    };

    let filtered = filter_advisories(advisories, severity_threshold, language);

    let total = filtered.len();
    write_notification(&json!({
        "type": "progress",
        "current": 0,
        "total": total
    }));

    for (i, advisory) in filtered.iter().enumerate() {
        write_notification(&json!({
            "type": "finding",
            "current": i + 1,
            "total": total,
            "data": advisory
        }));
    }

    let result = json!({
        "clean": filtered.is_empty(),
        "advisories": serde_json::to_value(&filtered).unwrap_or_default(),
    });
    write_response(&rpc_result(id, result));
}

pub fn run_audit(path: &str, severity_threshold: &str, language: Option<&str>) -> Value {
    let target = Path::new(path);
    if !target.exists() {
        return json!({
            "clean": false,
            "advisories": [],
            "error": format!("path does not exist: {}", path)
        });
    }

    let mut engine = Engine::new();

    let advisories = match engine.run(target) {
        Ok(advisories) => advisories,
        Err(e) => {
            return json!({
                "clean": false,
                "advisories": [],
                "error": format!("analysis error: {}", e)
            });
        }
    };

    let filtered = filter_advisories(advisories, severity_threshold, language);

    json!({
        "clean": filtered.is_empty(),
        "advisories": serde_json::to_value(&filtered).unwrap_or_default(),
    })
}

pub fn severity_rank(s: Severity) -> u8 {
    match s {
        Severity::Critical => 3,
        Severity::Warning => 2,
        Severity::Info => 1,
    }
}
