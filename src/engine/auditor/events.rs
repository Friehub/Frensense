// SPDX-License-Identifier: MIT

use super::GenSenseAuditor;
use crate::semantics::SymbolRegistry;
use petgraph::graph::NodeIndex;
use std::path::Path;
use tree_sitter::Node;

impl GenSenseAuditor {
    pub fn traverse_for_events<'a>(
        &self,
        node: Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        path: &Path,
        content: &str,
        registry: &mut SymbolRegistry,
        last_event: Option<NodeIndex>,
    ) -> Option<NodeIndex> {
        let mut current_last = last_event;

        let (event_type, label) = match node.kind() {
            "call_expression" | "macro_invocation" => {
                let fn_node = node
                    .child_by_field_name("function")
                    .or_else(|| node.child_by_field_name("macro"))
                    .or_else(|| node.child(0));

                if let Some(f) = fn_node {
                    let full_name = &content[f.start_byte()..f.end_byte()];

                    let base_name = if f.kind() == "field_expression" {
                        if let Some(field) = f.child_by_field_name("field") {
                            &content[field.start_byte()..field.end_byte()]
                        } else {
                            full_name
                        }
                    } else if f.kind() == "scoped_identifier" {
                        if let Some(name) = f.child_by_field_name("name") {
                            &content[name.start_byte()..name.end_byte()]
                        } else {
                            full_name
                        }
                    } else {
                        full_name
                    };

                    let mut normalized_name = base_name;
                    if normalized_name.ends_with('!') {
                        normalized_name = &normalized_name[..normalized_name.len() - 1];
                    }

                    let et = match normalized_name {
                        "lock" | "try_lock" | "acquire" | "wait" => {
                            crate::semantics::graph::EventType::Acquire
                        }
                        "unlock" | "release" | "drop" | "signal" => {
                            crate::semantics::graph::EventType::Release
                        }
                        _ => crate::semantics::graph::EventType::Call,
                    };
                    (Some(et), normalized_name.to_string())
                } else {
                    (None, String::new())
                }
            }
            "variable_declarator" | "let_declaration" => {
                let name_node = node
                    .child_by_field_name("name")
                    .or_else(|| node.child_by_field_name("pattern"));
                let name = name_node
                    .map(|n| &content[n.start_byte()..n.end_byte()])
                    .unwrap_or("");
                (
                    Some(crate::semantics::graph::EventType::Assignment),
                    name.to_string(),
                )
            }
            "assignment_expression" => {
                let left_node = node.child_by_field_name("left");
                let name = left_node
                    .map(|n| &content[n.start_byte()..n.end_byte()])
                    .unwrap_or("");
                (
                    Some(crate::semantics::graph::EventType::Assignment),
                    name.to_string(),
                )
            }
            "await_expression" => (
                Some(crate::semantics::graph::EventType::Await),
                ".await".to_string(),
            ),
            "return_statement" => (
                Some(crate::semantics::graph::EventType::Return),
                "return".to_string(),
            ),
            _ => (None, String::new()),
        };

        if let Some(et) = event_type {
            let event = crate::semantics::graph::TemporalEvent {
                event_type: et.clone(),
                label: label.clone(),
                file_path: path.to_string_lossy().to_string(),
                line: node.start_position().row + 1,
                column: node.start_position().column + 1,
            };
            let idx = registry.graph.add_event(event);

            if let Some(prev) = last_event {
                registry.graph.add_edge(
                    prev,
                    idx,
                    crate::semantics::graph::EdgeKind::SequentiallyFollows,
                );
            }

            if let Some(func) = self.find_enclosing_function(node) {
                if let Some(name_node) = func.child_by_field_name("name") {
                    let name = &content[name_node.start_byte()..name_node.end_byte()];
                    for &func_idx in &registry.graph.find_nodes(name) {
                        if let Some(sym) = registry.graph.get_symbol(func_idx) {
                            if sym.file_path == path.to_string_lossy() {
                                registry.graph.add_edge(
                                    func_idx,
                                    idx,
                                    crate::semantics::graph::EdgeKind::InScope,
                                );
                            }
                        }
                    }
                }
            }

            match et {
                crate::semantics::graph::EventType::Assignment => {
                    let value_node = node
                        .child_by_field_name("value")
                        .or_else(|| node.child_by_field_name("right"));
                    if let Some(v) = value_node {
                        let mut val_name = &content[v.start_byte()..v.end_byte()];

                        if v.kind() == "call_expression" {
                            if let Some(f) = v.child_by_field_name("function") {
                                val_name = &content[f.start_byte()..f.end_byte()];
                            }
                        } else if v.kind() == "macro_invocation" {
                            if let Some(m) = v.child_by_field_name("macro") {
                                val_name = &content[m.start_byte()..m.end_byte()];
                                if val_name.ends_with('!') {
                                    val_name = &val_name[..val_name.len() - 1];
                                }
                            }
                        }

                        let nodes = registry.graph.find_nodes(val_name);
                        for &v_idx in &nodes {
                            registry.graph.add_edge(
                                v_idx,
                                idx,
                                crate::semantics::graph::EdgeKind::FlowsFrom,
                            );
                        }
                    }
                    let targets = registry.graph.find_nodes(&label);
                    for &target_idx in &targets {
                        registry.graph.add_edge(
                            idx,
                            target_idx,
                            crate::semantics::graph::EdgeKind::FlowsFrom,
                        );
                    }
                }
                crate::semantics::graph::EventType::Call => {
                    let args_node = node
                        .child_by_field_name("arguments")
                        .or_else(|| node.child_by_field_name("token_tree"))
                        .or_else(|| {
                            let mut cursor = node.walk();
                            if cursor.goto_first_child() {
                                loop {
                                    let c = cursor.node();
                                    if c.kind() == "token_tree" || c.kind() == "arguments" {
                                        return Some(c);
                                    }
                                    if !cursor.goto_next_sibling() {
                                        break;
                                    }
                                }
                            }
                            None
                        });

                    if let Some(args) = args_node {
                        let mut arg_cursor = args.walk();
                        let mut p_idx = 0;
                        for arg in args.children(&mut arg_cursor) {
                            if matches!(arg.kind(), "(" | ")" | "," | "[" | "!" | "{") {
                                continue;
                            }
                            let arg_name = &content[arg.start_byte()..arg.end_byte()];

                            for &a_idx in &registry.graph.find_nodes(arg_name) {
                                registry.graph.add_edge(
                                    a_idx,
                                    idx,
                                    crate::semantics::graph::EdgeKind::FlowsFrom,
                                );

                                for &f_idx in &registry.graph.find_nodes(&label) {
                                    let params = registry.graph.neighbors_of(
                                        f_idx,
                                        crate::semantics::graph::EdgeKind::Parameter,
                                    );
                                    if let Some(&p_node_idx) = params.get(p_idx) {
                                        registry.graph.add_edge(
                                            a_idx,
                                            p_node_idx,
                                            crate::semantics::graph::EdgeKind::FlowsFrom,
                                        );
                                    }
                                }
                            }
                            p_idx += 1;
                        }
                    }
                }
                _ => {}
            }

            current_last = Some(idx);
        }

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                let child_last = if matches!(
                    child.kind(),
                    "function_item" | "function_declaration" | "method_definition"
                ) {
                    None
                } else {
                    current_last
                };

                let result_last =
                    self.traverse_for_events(child, cursor, path, content, registry, child_last);

                if !matches!(
                    child.kind(),
                    "function_item" | "function_declaration" | "method_definition"
                ) {
                    current_last = result_last;
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }

        current_last
    }
}
