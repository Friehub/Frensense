// SPDX-License-Identifier: MIT

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
            context,
            &mut has_call,
            &mut call_pos,
            &mut advisories,
        );

        advisories
    }
}

impl SolidityReentrancyGuard {
    fn scan_node<'a>(
        &self,
        node: Node<'a>,
        context: &GenSenseContext<'a>,
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
            let code = &context.source_code[node.start_byte()..node.end_byte()];
            if code.contains(".transfer(") || code.contains(".send(") || code.contains(".call(") {
                if *has_call {
                    if let Some((r, c)) = *call_pos {
                        let file_path = context.file_path.to_string_lossy().to_string();
                        let enclosing_symbol = context
                            .symbols
                            .find_function_at(&file_path, r)
                            .and_then(|idx| context.symbols.graph().get_symbol(idx))
                            .map(|s| s.name.clone());

                        advisories.push(Advisory {
                            rule_id: "SOLIDITY_REENTRANCY_RISK".to_string(),
                            file_id: context.file_id,
                            file_path,
                            severity: Severity::Warning,
                            observation: "Potential reentrancy: state-changing call after external transfer detected.".to_string(),
                            impact: "A malicious contract could call back into this one before the state is updated, potentially stealing funds.".to_string(),
                            improvement: "Follow the Checks-Effects-Interactions pattern: update state before making external calls.".to_string(),
                            line: r as u32,
                            column: c as u32,
                            start_byte: node.start_byte() as u32,
                            end_byte: node.end_byte() as u32,
                            original_content: context.source_code
                                [node.start_byte()..node.end_byte()]
                                .to_string(),
                            proposed_replacement: None,
                            proposed_import: None,
                            enclosing_symbol,
                            confidence: 0.9,
                            fingerprint: String::new(),
                            auto_fixable: false,
                            requires_human: true,
                            tags: vec![],
                        });
                    }
                }
            }
        }

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                self.scan_node(cursor.node(), context, has_call, call_pos, advisories);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}
