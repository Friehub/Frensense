// SPDX-License-Identifier: MIT

use tree_sitter::Node;

use crate::cfg;
use super::{PatternFinding, SemanticPattern};

pub struct MissingAuthGuard;

impl MissingAuthGuard {
    const SINK_CALLS: &[&str] = &[
        "send", "json", "redirect",
        "write", "writeFile", "writeFileSync",
        "query", "execute", "executeRaw", "queryRaw", "sql_query", "prepare",
        "exec", "spawn", "execSync", "spawnSync",
        "insert", "update", "delete", "save", "put",
    ];

    fn block_contains_sink(block_text: &str) -> bool {
        let lower = block_text.to_lowercase();
        Self::SINK_CALLS.iter().any(|sink| {
            let with_paren = format!("{sink}(");
            lower.contains(&with_paren)
        })
    }
}

impl SemanticPattern for MissingAuthGuard {
    fn id(&self) -> &str {
        "MISSING_AUTH_GUARD"
    }

    fn description(&self) -> &str {
        "Function performs state-changing operations without an authentication guard"
    }

    fn severity(&self) -> &str {
        "Critical"
    }

    fn languages(&self) -> &[&str] {
        &["*"]
    }

    fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding> {
        let ext = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let mut findings = Vec::new();
        let mut cursor = tree.walk();

        let function_kinds: &[&str] = &[
            "function_declaration",
            "method_definition",
            "function_item",
            "function_definition",
            "arrow_function",
        ];

        loop {
            let node = cursor.node();
            if function_kinds.contains(&node.kind()) {
                let body = node.child_by_field_name("body");
                if let Some(body_node) = body {
                    let mut cfg_graph = cfg::build_cfg(body_node, source, ext);
                    cfg::compute_dominators(&mut cfg_graph);

                    for block in &cfg_graph.blocks {
                        if block.start_byte >= source.len() || block.end_byte > source.len() {
                            continue;
                        }
                        let block_text = &source[block.start_byte..block.end_byte];

                        if Self::block_contains_sink(block_text) {
                            let has_auth = cfg::has_auth_guard_dominator(
                                &cfg_graph,
                                block.id,
                                source,
                            );

                            if !has_auth {
                                let fn_name = node
                                    .child_by_field_name("name")
                                    .and_then(|n| n.utf8_text(source.as_bytes()).ok())
                                    .unwrap_or("<anonymous>")
                                    .to_string();

                                let line = source[..block.start_byte].lines().count() + 1;
                                let col = source[..block.start_byte]
                                    .rfind('\n')
                                    .map_or(block.start_byte + 1, |i| block.start_byte - i);

                                findings.push(PatternFinding {
                                    pattern_id: self.id().to_string(),
                                    severity: self.severity().to_string(),
                                    line,
                                    column: col,
                                    observation: format!(
                                        "Function `{}` performs state-changing operations without an authentication guard",
                                        fn_name,
                                    ),
                                    impact: "Unauthenticated access to state-changing operations can lead to data breaches and unauthorized modifications".to_string(),
                                    improvement: "Add an authentication check (e.g. session validation, token verification) before the state-changing operation".to_string(),
                                    confidence: 0.85,
                                    tags: vec![
                                        "security".to_string(),
                                        "auth".to_string(),
                                        "access-control".to_string(),
                                    ],
                                    enclosing_function: Some(fn_name),
                                });

                                break;
                            }
                        }
                    }
                }
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return findings;
                }
            }
        }
    }
}