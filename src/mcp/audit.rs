#![allow(
    clippy::must_use_candidate,
    clippy::print_stderr,
    clippy::missing_panics_doc
)]
//! Audit orchestration logic for the MCP server.

use super::protocol::{RequestId, rpc_result, write_response};
use crate::{Advisory, Engine, Severity};
use serde_json::{Value, json};
use std::io::{self, Write};
use std::path::Path;

pub fn tool_definition() -> Value {
    json!({
        "name": "frensense_audit",
        "description": "Run semantic analysis on a file or directory. Returns advisories the agent must resolve before code is considered correct. An empty advisories array and clean=true means the code satisfies all invariants. When stream=true, findings are sent as notifications followed by a final result.",
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
                },
                "stream": {
                    "type": "boolean",
                    "default": false,
                    "description": "Emit findings as JSON-RPC notifications for progressive display"
                },
                "language": {
                    "type": "string",
                    "description": "Filter by language extension (rust, typescript)"
                },
                "rules": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Only include these rule IDs"
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
    rules: Option<&[String]>,
) -> Vec<Advisory> {
    let threshold = match severity_threshold {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    };

    let extensions = language.and_then(crate::parser::ParserRegistry::extensions_for);

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
        .filter(|a| {
            if let Some(rules) = rules {
                rules.contains(&a.rule_id)
            } else {
                true
            }
        })
        .collect()
}

pub fn run_audit_streamed(
    id: RequestId,
    path: &str,
    fix_auto: bool,
    severity_threshold: &str,
    language: Option<&str>,
    rules: Option<&[String]>,
) {
    let target = Path::new(path);
    if !target.exists() {
        let result = json!({
            "clean": false,
            "advisories": [],
            "auto_fixed": 0,
            "requires_human": [],
            "error": format!("path does not exist: {}", path)
        });
        write_response(&rpc_result(id, result));
        return;
    }

    let mut engine = Engine::new();
    let rule_count = engine.auditor().rules().len();
    eprintln!(
        "frensense-mcp: streaming cwd={:?}, rules={}",
        std::env::current_dir().ok(),
        rule_count,
    );

    let advisories = match engine.run(target) {
        Ok(a) => a,
        Err(e) => {
            let result = json!({
                "clean": false,
                "advisories": [],
                "auto_fixed": 0,
                "requires_human": [],
                "error": format!("analysis error: {}", e)
            });
            write_response(&rpc_result(id, result));
            return;
        }
    };

    let filtered = filter_advisories(advisories, severity_threshold, language, rules);

    let total = filtered.len();
    write_notification(&json!({
        "type": "progress",
        "current": 0,
        "total": total
    }));

    let mut auto_fixable_count = 0u64;
    let mut requires_human: Vec<Advisory> = Vec::new();

    for (i, advisory) in filtered.iter().enumerate() {
        if advisory.proposed_replacement.is_some() {
            auto_fixable_count += 1;
        }
        if advisory.requires_human || advisory.proposed_replacement.is_none() {
            requires_human.push(advisory.clone());
        }
        write_notification(&json!({
            "type": "finding",
            "current": i + 1,
            "total": total,
            "data": advisory
        }));
    }

    if fix_auto {
        apply_auto_fixes(&filtered, target);
    }

    let result = json!({
        "clean": filtered.is_empty(),
        "advisories": serde_json::to_value(&filtered).unwrap_or_default(),
        "auto_fixed": auto_fixable_count,
        "requires_human": serde_json::to_value(&requires_human).unwrap_or_default()
    });
    write_response(&rpc_result(id, result));
}

pub fn run_audit(
    path: &str,
    fix_auto: bool,
    severity_threshold: &str,
    language: Option<&str>,
    rules: Option<&[String]>,
) -> Value {
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

    let mut engine = Engine::new();
    let rule_count = engine.auditor().rules().len();
    eprintln!(
        "frensense-mcp: cwd={:?}, rules={}, threshold={:?}",
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

    let filtered = filter_advisories(advisories, severity_threshold, language, rules);

    let auto_fixable_count = filtered
        .iter()
        .filter(|a| a.proposed_replacement.is_some())
        .count() as u64;

    let requires_human: Vec<Advisory> = filtered
        .iter()
        .filter(|a| a.requires_human || a.proposed_replacement.is_none())
        .cloned()
        .collect();

    if fix_auto {
        apply_auto_fixes(&filtered, target);
    }

    json!({
        "clean": filtered.is_empty(),
        "advisories": serde_json::to_value(&filtered).unwrap_or_default(),
        "auto_fixed": auto_fixable_count,
        "requires_human": serde_json::to_value(&requires_human).unwrap_or_default()
    })
}

pub fn severity_rank(s: Severity) -> u8 {
    match s {
        Severity::Critical => 3,
        Severity::Warning => 2,
        Severity::Info => 1,
    }
}

pub fn apply_auto_fixes(advisories: &[Advisory], root: &Path) {
    use crate::patcher::PatchManager;

    let project_root = find_project_root_for_fix(root);
    let patcher = PatchManager::new(&project_root);

    let mut fixable: Vec<&Advisory> = advisories
        .iter()
        .filter(|a| a.proposed_replacement.is_some())
        .collect();

    fixable.sort_by_key(|a| std::cmp::Reverse(a.start_byte));

    for adv in &fixable {
        let _ = patcher.apply_fix(adv, Path::new(&adv.file_path));
    }
}

pub fn find_project_root_for_fix(target: &Path) -> std::path::PathBuf {
    let mut root = target.to_path_buf();
    if root.is_file() {
        root = root.parent().unwrap_or(&root).to_path_buf();
    }
    while root.parent().is_some() {
        if root.join(".frensense").exists() || root.join(".git").exists() {
            break;
        }
        root = root.parent().expect("parent").to_path_buf();
    }
    root
}
