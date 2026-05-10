// SPDX-License-Identifier: MIT

use super::{DataFlowAnalyzer, TaintRegistry};
use crate::{Advisory, GenSenseRule};
use regex::Regex;
use tree_sitter::Node;

impl<'a> DataFlowAnalyzer<'a> {
    /// DEPRECATED: Use `check_taint_graph` for more robust inter-procedural analysis.
    pub fn analyze_block(
        &self,
        block: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        mut registry: TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut cursor = block.walk();

        for child in block.children(&mut cursor) {
            match child.kind() {
                "variable_declarator" | "let_declaration" | "lexical_declaration" => {
                    self.resolve_variable(child, source_re, &mut registry);
                }
                "assignment_expression" => {
                    let left = child.child_by_field_name("left");
                    let right = child.child_by_field_name("right");
                    if let (Some(l), Some(r)) = (left, right) {
                        let r_code = &self.context.source_code[r.start_byte()..r.end_byte()];
                        if let Some(origin) = registry.get_origin(r_code) {
                            let l_code = &self.context.source_code[l.start_byte()..l.end_byte()];
                            registry.taint(l_code.to_string(), origin);
                        } else if source_re.is_match(r_code) {
                            let l_code = &self.context.source_code[l.start_byte()..l.end_byte()];
                            registry.taint(l_code.to_string(), r_code.to_string());
                        }
                    }
                }
                "expression_statement" => {
                    if let Some(expr) = child.child(0) {
                        if expr.kind() == "call_expression" {
                            advisories.extend(self.resolve_call_expression(
                                expr,
                                source_re,
                                sink_re,
                                rule,
                                &mut registry,
                            ));
                        }
                    }
                }
                "call_expression" => {
                    advisories.extend(self.resolve_call_expression(
                        child,
                        source_re,
                        sink_re,
                        rule,
                        &mut registry,
                    ));
                }
                _ => {}
            }
        }
        advisories
    }

    pub fn resolve_variable(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        registry: &mut TaintRegistry,
    ) {
        let name_node = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("pattern"));
        let value_node = node
            .child_by_field_name("value")
            .or_else(|| node.child_by_field_name("right"));

        if let (Some(n), Some(v)) = (name_node, value_node) {
            let name = &self.context.source_code[n.start_byte()..n.end_byte()];
            let value = &self.context.source_code[v.start_byte()..v.end_byte()];

            if let Some(origin) = registry.get_origin(value) {
                registry.taint(name.to_string(), origin);
            } else if source_re.is_match(value) {
                registry.taint(name.to_string(), value.to_string());
            }
        }
    }

    pub fn resolve_call_expression(
        &self,
        node: Node<'a>,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
        registry: &mut TaintRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let fn_node = node.child_by_field_name("function");
        let args_node = node.child_by_field_name("arguments");

        if let (Some(f), Some(args)) = (fn_node, args_node) {
            let fn_name = &self.context.source_code[f.start_byte()..f.end_byte()];
            let mut tainted_args = Vec::new();
            let mut arg_idx = 0;
            let mut cursor = args.walk();

            for arg in args.children(&mut cursor) {
                if matches!(arg.kind(), "(" | ")" | ",") {
                    continue;
                }
                let arg_code = &self.context.source_code[arg.start_byte()..arg.end_byte()];
                if let Some(origin) = registry.get_origin(arg_code) {
                    tainted_args.push(arg_idx);
                    if sink_re.is_match(fn_name) {
                        advisories.push(rule.new_advisory(
                            &arg,
                            format!("Inter-procedural Leak: Tainted data from '{origin}' reached sink '{fn_name}' via variable '{arg_code}'."),
                            rule.impact().to_string(),
                            rule.improvement().to_string(),
                        ));
                    }
                }
                arg_idx += 1;
            }

            if !tainted_args.is_empty() && self.depth < self.max_depth {
                if let Some(def_node) = self.find_definition(fn_name) {
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
        }
        advisories
    }

    pub fn check_taint_graph(
        &self,
        source_re: &Regex,
        sink_re: &Regex,
        rule: &dyn GenSenseRule,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let graph = &self.context.symbols.graph;

        // 1. Find source nodes
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut source_map = std::collections::HashMap::new();

        for idx in graph.all_nodes() {
            if let Some(node) = graph.get_node(idx) {
                let label = match node {
                    crate::semantics::graph::SemanticNode::Declaration(s) => &s.name,
                    crate::semantics::graph::SemanticNode::Event(e) => &e.label,
                };
                if source_re.is_match(label) {
                    queue.push_back(idx);
                    visited.insert(idx);
                    source_map.insert(idx, label.clone());
                }
            }
        }

        // 2. BFS for reachability
        while let Some(current_idx) = queue.pop_front() {
            let source_label = source_map.get(&current_idx).cloned().unwrap_or_default();

            if let Some(node) = graph.get_node(current_idx) {
                let label = match node {
                    crate::semantics::graph::SemanticNode::Declaration(s) => &s.name,
                    crate::semantics::graph::SemanticNode::Event(e) => &e.label,
                };
                if sink_re.is_match(label) {
                    let (line, col, path) = match node {
                        crate::semantics::graph::SemanticNode::Declaration(s) => {
                            (s.line, s.column, s.file_path.clone())
                        }
                        crate::semantics::graph::SemanticNode::Event(e) => {
                            (e.line, e.column, e.file_path.clone())
                        }
                    };

                    advisories.push(Advisory {
                        rule_id: rule.id().to_string(),
                        severity: rule.severity(),
                        observation: format!(
                            "Inter-procedural Leak: Tainted data from '{source_label}' reached sink '{label}'."
                        ),
                        impact: rule.impact().to_string(),
                        improvement: rule.improvement().to_string(),
                        line,
                        column: col,
                        file_path: path,
                        original_content: String::new(),
                        proposed_replacement: None,
                    });
                }
            }

            // FIX (Bug 3): Replaced EdgeKind::InScope with EdgeKind::Calls.
            //
            // The original array contained InScope, which links a function node
            // to every event inside it. This caused catastrophic over-taint:
            // any source event inside a function would propagate via InScope to
            // the function node, then back out via InScope to ALL other events
            // in that function, regardless of actual data flow. One tainted call
            // would contaminate an entire function's worth of nodes.
            //
            // EdgeKind::Calls is the correct interprocedural edge: it connects
            // a caller to a callee and was already present in the graph but
            // never used by this BFS. Adding it here enables real cross-function
            // taint propagation — a source in one function now correctly flows
            // into functions it calls — without the whole-scope contamination.
            let next_edges = [
                crate::semantics::graph::EdgeKind::FlowsFrom,
                crate::semantics::graph::EdgeKind::Calls, // FIX: was InScope
                crate::semantics::graph::EdgeKind::SequentiallyFollows,
            ];
            for &kind in &next_edges {
                for next_idx in graph.neighbors_of(current_idx, kind) {
                    if visited.insert(next_idx) {
                        source_map.insert(next_idx, source_label.clone());
                        queue.push_back(next_idx);
                    }
                }
            }
        }

        advisories
    }
}
