// SPDX-License-Identifier: MIT

use crate::{Advisory, FrensenseContext, FrensenseRule, RuleMetadata, Severity};
use std::borrow::Cow;
use std::sync::OnceLock;
use tree_sitter::Node;

pub struct DeadlockGuard;

static METADATA: OnceLock<RuleMetadata> = OnceLock::new();

impl FrensenseRule for DeadlockGuard {
    fn metadata(&self) -> &RuleMetadata {
        METADATA.get_or_init(|| RuleMetadata {
            id: Cow::Borrowed("RUST_ASYNC_MUTEX_DEADLOCK"),
            name: Cow::Borrowed("Async Mutex Deadlock Detector"),
            severity: Severity::Critical,
            observation: Cow::Borrowed("Potential async deadlock detected: Mutex guard held across .await point."),
            impact: Cow::Borrowed("Holding a standard Mutex guard across an await point can block the entire executor thread."),
            improvement: Cow::Borrowed("Use tokio::sync::Mutex or ensure the guard is dropped before the await."),
            tags: vec![Cow::Borrowed("reliability"), Cow::Borrowed("async"), Cow::Borrowed("rust")],
            category: Cow::Borrowed("Reliability"),
            confidence: 0.85,
            precision: crate::Precision::VeryHigh,
        })
    }

    fn applies_to(&self, ext: &str) -> bool {
        crate::parser::ParserRegistry::ext_matches(ext, &["rs"])
    }

    fn query(&self) -> Option<&str> {
        Some("(function_item) @node")
    }

    #[allow(clippy::cast_possible_truncation)]
    fn check<'a>(&self, node: Node<'a>, context: &FrensenseContext<'a>) -> Vec<Advisory> {
        let source = context.source_code;
        let Some(body) = node.child_by_field_name("body") else {
            return Vec::new();
        };

        let header = &source[node.start_byte()..body.start_byte()];
        if !header.contains("async") {
            return Vec::new();
        }

        let mut locks: Vec<Node<'a>> = Vec::new();
        let mut awaits: Vec<Node<'a>> = Vec::new();
        collect_locks_and_awaits(body, source, &mut locks, &mut awaits);

        if locks.is_empty() || awaits.is_empty() {
            return Vec::new();
        }

        locks.sort_by_key(tree_sitter::Node::start_byte);
        awaits.sort_by_key(tree_sitter::Node::start_byte);

        let meta = self.metadata();
        let file_path = context.file_path.to_string_lossy().to_string();
        let file_id = context.file_id;
        let mut advisories = Vec::new();
        let mut last_flagged_line: Option<u32> = None;

        for lock_node in &locks {
            let block_end = enclosing_block_end(*lock_node);
            let lock_line = (lock_node.start_position().row + 1) as u32;

            for await_node in &awaits {
                let await_start = await_node.start_byte();
                let await_line = (await_node.start_position().row + 1) as u32;

                if await_start <= lock_node.start_byte() {
                    continue;
                }

                if await_start >= block_end {
                    break;
                }

                if last_flagged_line == Some(await_line) {
                    continue;
                }

                last_flagged_line = Some(await_line);

                advisories.push(Advisory {
                    rule_id: meta.id.to_string(),
                    file_id,
                    file_path: file_path.clone(),
                    severity: meta.severity,
                    confidence: 0.92,
                    observation: format!(
                        "Potential deadlock: Mutex guard locked at line {lock_line} is still held across .await at line {await_line}.",
                    ),
                    impact: meta.impact.to_string(),
                    improvement: meta.improvement.to_string(),
                    line: await_line,
                    column: (await_node.start_position().column + 1) as u32,
                    start_byte: 0,
                    end_byte: 0,
                    original_content: source[await_node.start_byte()..await_node.end_byte()].to_string(),
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: None,
                    fingerprint: String::new(),
                    auto_fixable: false,
                    requires_human: true,
                    tags: meta.tags.iter().map(ToString::to_string).collect(),
                });
            }
        }

        advisories
    }
}

fn collect_locks_and_awaits<'a>(
    node: Node<'a>,
    source: &'a str,
    locks: &mut Vec<Node<'a>>,
    awaits: &mut Vec<Node<'a>>,
) {
    if node.kind() == "await_expression" {
        awaits.push(node);
    } else if node.kind() == "call_expression" {
        if let Some(parent) = node.parent()
            && parent.kind() == "await_expression"
        {
            return;
        }
        if let Some(func) = node.child_by_field_name("function")
            && let Some(field) = func.child_by_field_name("field")
            && &source[field.start_byte()..field.end_byte()] == "lock"
        {
            locks.push(node);
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_locks_and_awaits(cursor.node(), source, locks, awaits);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn enclosing_block_end(node: Node) -> usize {
    let mut current = node;
    loop {
        match current.parent() {
            Some(p) if p.kind() == "block" => return p.end_byte(),
            Some(p) => current = p,
            None => return node.end_byte(),
        }
    }
}
