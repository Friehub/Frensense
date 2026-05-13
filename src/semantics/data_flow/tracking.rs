// SPDX-License-Identifier: MIT

use super::normalization::SemanticOp;
use super::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a, 'ctx> DataFlowAnalyzer<'a, 'ctx> {
    pub fn discover_symbols(&self, registry: &mut TaintRegistry<'a>) {
        for op in self.context.semantic_ops {
            if let SemanticOp::Binding { name, value_range } = op {
                // Register symbol with its range
                let node = self.node_at(*value_range);
                registry.register_symbol(name, node);
            }
        }
    }

    fn node_at(&self, range: crate::semantics::data_flow::normalization::Range) -> Node<'a> {
        self.context
            .tree
            .root_node()
            .descendant_for_byte_range(range.start_byte, range.end_byte)
            .unwrap_or_else(|| self.context.tree.root_node())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn analyze_block(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        mut registry: TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let block_range = (node.start_byte(), node.end_byte());

        for op in self.context.semantic_ops {
            // Only process ops within the current block's range
            match op {
                SemanticOp::Binding { name, value_range } => {
                    if value_range.start_byte >= block_range.0
                        && value_range.end_byte <= block_range.1
                    {
                        let v_node = self.node_at(*value_range);
                        registry.register_symbol(name, v_node);
                        let v_code =
                            &self.context.source_code[v_node.start_byte()..v_node.end_byte()];
                        if source_re.is_match(v_code) {
                            registry.taint(name, "source");
                        } else if let Some(origin) = self.resolve_taint(v_node, &registry) {
                            registry.taint(name, origin);
                        }
                    }
                }
                SemanticOp::Assignment {
                    target,
                    value_range,
                } => {
                    if value_range.start_byte >= block_range.0
                        && value_range.end_byte <= block_range.1
                    {
                        let v_node = self.node_at(*value_range);
                        let v_code =
                            &self.context.source_code[v_node.start_byte()..v_node.end_byte()];
                        if source_re.is_match(v_code) {
                            registry.taint(target, "source");
                        } else if let Some(origin) = self.resolve_taint(v_node, &registry) {
                            registry.taint(target, origin);
                        }
                    }
                }
                SemanticOp::Call {
                    function_name,
                    args,
                    range,
                } => {
                    if range.start_byte >= block_range.0 && range.end_byte <= block_range.1 {
                        let arg_nodes: Vec<Node> = args.iter().map(|r| self.node_at(*r)).collect();
                        let call_node = self.node_at(*range);
                        advisories.extend(self.analyze_call(
                            function_name,
                            &arg_nodes,
                            call_node,
                            source_re,
                            sink_re,
                            rule,
                            &mut registry,
                        ));
                    }
                }
                SemanticOp::EnterBlock(body_range) => {
                    if body_range.start_byte > block_range.0
                        && body_range.end_byte < block_range.1
                        && self.depth < self.max_depth
                    {
                        let body_node = self.node_at(*body_range);
                        registry.push_scope();
                        let sub_analyzer = DataFlowAnalyzer::with_depth(
                            self.context,
                            body_node,
                            self.depth + 1,
                            self.max_depth,
                        );
                        advisories.extend(sub_analyzer.analyze_block(
                            body_node,
                            source_re,
                            sink_re,
                            rule,
                            registry.clone(),
                        ));
                        registry.pop_scope();
                    }
                }
            }
        }

        advisories
    }

    #[allow(clippy::too_many_arguments)]
    fn analyze_call(
        &self,
        fn_name: &'a str,
        args: &[Node<'a>],
        _call_node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut tainted_args = Vec::new();

        for (idx, arg) in args.iter().enumerate() {
            if let Some(origin) = self.resolve_taint(*arg, registry) {
                tainted_args.push(idx);
                if sink_re.is_match(fn_name) {
                    let arg_code = &self.context.source_code[arg.start_byte()..arg.end_byte()];
                    advisories.push(rule.new_advisory(
                        arg,
                        self.context,
                        format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                    ));
                }
            }
        }

        if !tainted_args.is_empty() && self.depth < self.max_depth {
            if let Some(def_node) = self.find_definition(fn_name, registry) {
                if let Some(next_registry) = self.map_params(def_node, &tainted_args) {
                    if let Some(body) = def_node.child_by_field_name("body") {
                        let sub_analyzer = DataFlowAnalyzer::with_depth(
                            self.context,
                            body,
                            self.depth + 1,
                            self.max_depth,
                        );
                        advisories.extend(sub_analyzer.analyze_block(
                            body,
                            source_re,
                            sink_re,
                            rule,
                            next_registry,
                        ));
                    }
                }
            }
        }

        advisories
    }

    fn resolve_taint(&self, node: Node<'a>, registry: &TaintRegistry<'a>) -> Option<&'a str> {
        if node.kind() == "identifier" {
            let name = &self.context.source_code[node.start_byte()..node.end_byte()];
            return registry.get_origin(name);
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if let Some(origin) = self.resolve_taint(child, registry) {
                return Some(origin);
            }
        }
        None
    }
}
