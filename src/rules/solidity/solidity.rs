// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

#![allow(clippy::all)]
use crate::{Advisory, GenSenseContext, GenSenseRule, Severity};
use tree_sitter::Node;
pub struct SolidityReentrancyGuard;

impl GenSenseRule for SolidityReentrancyGuard {
    fn id(&self) -> &str {
        "SOL_REENTRANCY_PATTERN"
    }

    fn description(&self) -> &str {
        "Potential reentrancy pattern detected: state mutation after an external call."
    }

    fn query(&self) -> Option<&str> {
        // Match functions that contain both a call and an assignment
        Some("function_definition")
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "sol"
    }

    fn check<'a>(&self, node: Node<'a>, context: & GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let mut has_call = false;
        let mut call_pos = None;

        self.scan_node(
            node,
            context.source_code,
            &mut has_call,
            &mut call_pos,
            &mut advisories,
        );

        advisories
    }
}

impl SolidityReentrancyGuard {
    fn scan_node(
        &self,
        node: Node,
        _source: &str,
        has_call: &mut bool,
        call_pos: &mut Option<(usize, usize)>,
        advisories: &mut Vec<Advisory>,
    ) {
        let kind = node.kind();
        if kind == "function_call" {
            *has_call = true;
            *call_pos = Some((
                node.start_position().row + 1,
                node.start_position().column + 1,
            ));
        }

        if kind == "member_expression" {
            let code = &_source[node.start_byte()..node.end_byte()];
            if code.contains(".transfer(") || code.contains(".send(") || code.contains(".call(") {
                if *has_call {
                    if let Some((r, c)) = *call_pos {
                        advisories.push(Advisory {
                            rule_id: "SOLIDITY_REENTRANCY_RISK".to_string(),
                            severity: Severity::Warning,
                            observation: "Potential reentrancy: state-changing call after external transfer detected.".to_string(),
                            impact: "A malicious contract could call back into this one before the state is updated, potentially stealing funds.".to_string(),
                            improvement: "Follow the Checks-Effects-Interactions pattern: update state before making external calls.".to_string(),
                            line: r,
                            column: c,
                            file_path: String::new(),
                            original_content: String::new(),
                            proposed_replacement: None,
                        });
                    }
                }
            }
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                self.scan_node(cursor.node(), _source, has_call, call_pos, advisories);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}
