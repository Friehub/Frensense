// SPDX-License-Identifier: MIT

use super::normalization::SemanticOp;
use super::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a, '_> {
    pub fn discover_symbols(&self, registry: &mut TaintRegistry) {
        for op in self.context.semantic_ops {
            if let SemanticOp::Binding { name, value_range } = op {
                registry.register_symbol(name, value_range.start_byte, value_range.end_byte);
            }
        }
    }

    pub fn analyze_block(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let block_range = super::normalization::Range::from(node);

        for op in self.context.semantic_ops {
            match op {
                SemanticOp::Binding { name, value_range } => {
                    self.process_binding(
                        name,
                        *value_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                        &mut advisories,
                    );
                }
                SemanticOp::Assignment {
                    target,
                    value_range,
                } => {
                    self.process_assignment(
                        target,
                        *value_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                        &mut advisories,
                    );
                }
                SemanticOp::Call {
                    function_name,
                    args,
                    range,
                } => {
                    if let Some(call_advisories) = self.process_call(
                        function_name,
                        args,
                        *range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                    ) {
                        advisories.extend(call_advisories);
                    }
                }
                SemanticOp::EnterBlock(body_range) => {
                    if let Some(sub_advisories) = self.process_enter_block(
                        *body_range,
                        block_range,
                        source_re,
                        sink_re,
                        rule,
                        registry,
                    ) {
                        advisories.extend(sub_advisories);
                    }
                }
            }
        }

        advisories
    }
}
