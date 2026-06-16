// SPDX-License-Identifier: MIT

use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub start_byte: usize,
    pub end_byte: usize,
}

impl From<tree_sitter::Node<'_>> for Range {
    fn from(node: tree_sitter::Node) -> Self {
        Self {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemanticOp {
    Binding {
        name: String,
        value_range: Range,
    },
    Assignment {
        target: String,
        value_range: Range,
    },
    Call {
        function_name: String,
        args: Vec<Range>,
        range: Range,
    },
    EnterBlock(Range),
}

pub struct SemanticExtractor;

impl SemanticExtractor {
    pub fn extract(node: Node, source: &str, ext: &str) -> Vec<SemanticOp> {
        let mut ops = Vec::new();
        match ext {
            "rs" => Self::extract_rust(node, source, &mut ops),
            "ts" | "js" | "tsx" | "jsx" => Self::extract_typescript(node, source, &mut ops),
            _ => {}
        }
        ops
    }

    fn extract_bindings(node: Node, source: &str, value_node: Node, ops: &mut Vec<SemanticOp>) {
        match node.kind() {
            "identifier" | "variable_declarator" => {
                let name = source[node.start_byte()..node.end_byte()].to_string();
                ops.push(SemanticOp::Binding {
                    name,
                    value_range: value_node.into(),
                });
            }
            "assignment_pattern" | "shorthand_property_identifier_pattern" => {
                Self::extract_bindings(
                    node.child_by_field_name("key").unwrap_or(node),
                    source,
                    value_node,
                    ops,
                );
            }
            _ => {}
        }
    }

    fn extract_rust(root: Node, source: &str, ops: &mut Vec<SemanticOp>) {
        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            let kind = node.kind();

            match kind {
                "call_expression" => {
                    let func = node.child_by_field_name("function");
                    let args_node = node.child_by_field_name("arguments");
                    let name = func
                        .map(|f| source[f.start_byte()..f.end_byte()].to_string())
                        .unwrap_or_default();
                    let args = args_node
                        .map(|a| {
                            let mut cursor = a.walk();
                            let mut result = Vec::new();
                            loop {
                                let child = cursor.node();
                                if child.kind() != "comment" {
                                    result.push(Range::from(child));
                                }
                                if !cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                            result
                        })
                        .unwrap_or_default();
                    ops.push(SemanticOp::Call {
                        function_name: name,
                        args,
                        range: node.into(),
                    });
                }
                "let_declaration" => {
                    if let Some(value) = node.child_by_field_name("value") {
                        if let Some(pattern) = node.child_by_field_name("pattern") {
                            Self::extract_bindings(pattern, source, value, ops);
                        }
                    }
                }
                "assignment_expression" => {
                    if let (Some(target), Some(value)) = (
                        node.child_by_field_name("left"),
                        node.child_by_field_name("right"),
                    ) {
                        let target_name =
                            source[target.start_byte()..target.end_byte()].to_string();
                        ops.push(SemanticOp::Assignment {
                            target: target_name,
                            value_range: value.into(),
                        });
                    }
                }
                _ => {}
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return;
                }
            }
        }
    }

    fn extract_typescript(root: Node, source: &str, ops: &mut Vec<SemanticOp>) {
        let mut cursor = root.walk();
        loop {
            let node = cursor.node();
            let kind = node.kind();

            match kind {
                "call_expression" => {
                    let func = node.child_by_field_name("function");
                    let args_node = node.child_by_field_name("arguments");
                    let name = func
                        .map(|f| source[f.start_byte()..f.end_byte()].to_string())
                        .unwrap_or_default();
                    let args = args_node
                        .map(|a| {
                            let mut cursor = a.walk();
                            let mut result = Vec::new();
                            loop {
                                let child = cursor.node();
                                if child.kind() != "comment" {
                                    result.push(Range::from(child));
                                }
                                if !cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                            result
                        })
                        .unwrap_or_default();
                    ops.push(SemanticOp::Call {
                        function_name: name,
                        args,
                        range: node.into(),
                    });
                }
                "lexical_declaration" | "variable_declaration" => {
                    if let Some(value) = node.child_by_field_name("value") {
                        if let Some(pattern) = node.child_by_field_name("pattern") {
                            Self::extract_bindings(pattern, source, value, ops);
                        }
                    }
                }
                "assignment_expression" => {
                    if let (Some(target), Some(value)) = (
                        node.child_by_field_name("left"),
                        node.child_by_field_name("right"),
                    ) {
                        let target_name =
                            source[target.start_byte()..target.end_byte()].to_string();
                        ops.push(SemanticOp::Assignment {
                            target: target_name,
                            value_range: value.into(),
                        });
                    }
                }
                _ => {}
            }

            if cursor.goto_first_child() {
                continue;
            }
            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return;
                }
            }
        }
    }
}
