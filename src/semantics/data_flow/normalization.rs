// SPDX-License-Identifier: MIT

use tree_sitter::Node;

/// Normalized Semantic Operations.
/// This layer decouples the raw AST from the Analysis Engine.
#[derive(Debug, Clone)]
pub enum SemanticOp<'a> {
    /// A new variable binding (e.g., const x = y, let a = 1)
    Binding { name: &'a str, value_node: Node<'a> },
    /// An assignment to an existing variable (e.g., x = z)
    Assignment {
        target: &'a str,
        value_node: Node<'a>,
    },
    /// A function or method call
    Call {
        function_name: &'a str,
        args: Vec<Node<'a>>,
        node: Node<'a>,
    },
    /// Entering a nested executable block (e.g., a function body, if block)
    EnterBlock(Node<'a>),
}

pub struct SemanticExtractor;

impl SemanticExtractor {
    /// Extracts normalized semantic operations from a language-specific AST node.
    pub fn extract<'a>(node: Node<'a>, source: &'a str, ext: &str) -> Vec<SemanticOp<'a>> {
        let mut ops = Vec::new();

        match ext {
            "ts" | "tsx" | "js" | "jsx" => {
                Self::extract_typescript(node, source, &mut ops);
            }
            "rs" => {
                Self::extract_rust(node, source, &mut ops);
            }
            _ => {}
        }

        ops
    }

    fn extract_typescript<'a>(node: Node<'a>, source: &'a str, ops: &mut Vec<SemanticOp<'a>>) {
        match node.kind() {
            "program" | "statement_block" | "internal_module" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::extract_typescript(child, source, ops);
                }
            }
            "lexical_declaration" | "variable_declaration" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        let name_node = child.child_by_field_name("name");
                        let value_node = child.child_by_field_name("value");
                        if let (Some(n), Some(v)) = (name_node, value_node) {
                            ops.push(SemanticOp::Binding {
                                name: &source[n.start_byte()..n.end_byte()],
                                value_node: v,
                            });
                        }
                    }
                }
            }
            "assignment_expression" => {
                let left = node.child_by_field_name("left");
                let right = node.child_by_field_name("right");
                if let (Some(l), Some(r)) = (left, right) {
                    ops.push(SemanticOp::Assignment {
                        target: &source[l.start_byte()..l.end_byte()],
                        value_node: r,
                    });
                }
            }
            "expression_statement" => {
                if let Some(expr) = node.child(0) {
                    Self::extract_typescript(expr, source, ops);
                }
            }
            "call_expression" => {
                let func = node.child_by_field_name("function");
                let args = node.child_by_field_name("arguments");
                if let (Some(f), Some(a)) = (func, args) {
                    let mut arg_list = Vec::new();
                    let mut cursor = a.walk();
                    for arg in a.children(&mut cursor) {
                        if !matches!(arg.kind(), "(" | ")" | ",") {
                            arg_list.push(arg);
                        }
                    }

                    let fn_name = &source[f.start_byte()..f.end_byte()];
                    ops.push(SemanticOp::Call {
                        function_name: fn_name,
                        args: arg_list,
                        node,
                    });
                }
            }
            "function_declaration" | "arrow_function" | "method_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    ops.push(SemanticOp::Binding {
                        name: &source[name_node.start_byte()..name_node.end_byte()],
                        value_node: node,
                    });
                }
                if let Some(body) = node.child_by_field_name("body") {
                    ops.push(SemanticOp::EnterBlock(body));
                }
            }
            _ => {}
        }
    }

    fn extract_rust<'a>(node: Node<'a>, source: &'a str, ops: &mut Vec<SemanticOp<'a>>) {
        match node.kind() {
            "source_file" | "block" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::extract_rust(child, source, ops);
                }
            }
            "let_declaration" => {
                let name_node = node.child_by_field_name("pattern");
                let value_node = node.child_by_field_name("value");
                if let (Some(n), Some(v)) = (name_node, value_node) {
                    ops.push(SemanticOp::Binding {
                        name: &source[n.start_byte()..n.end_byte()],
                        value_node: v,
                    });
                }
            }
            "assignment_expression" => {
                let left = node.child_by_field_name("left");
                let right = node.child_by_field_name("right");
                if let (Some(l), Some(r)) = (left, right) {
                    ops.push(SemanticOp::Assignment {
                        target: &source[l.start_byte()..l.end_byte()],
                        value_node: r,
                    });
                }
            }
            "expression_statement" => {
                if let Some(expr) = node.child(0) {
                    Self::extract_rust(expr, source, ops);
                }
            }
            "call_expression" => {
                let func = node.child_by_field_name("function");
                let args = node.child_by_field_name("arguments");
                if let (Some(f), Some(a)) = (func, args) {
                    let mut arg_list = Vec::new();
                    let mut cursor = a.walk();
                    for arg in a.children(&mut cursor) {
                        if !matches!(arg.kind(), "(" | ")" | ",") {
                            arg_list.push(arg);
                        }
                    }
                    ops.push(SemanticOp::Call {
                        function_name: &source[f.start_byte()..f.end_byte()],
                        args: arg_list,
                        node,
                    });
                }
            }
            "macro_invocation" => {
                let macro_name_node = node.child_by_field_name("macro");
                let args_node = node.child_by_field_name("arguments");
                if let (Some(m), Some(a)) = (macro_name_node, args_node) {
                    let mut arg_list = Vec::new();
                    let mut cursor = a.walk();
                    for arg in a.children(&mut cursor) {
                        if !matches!(arg.kind(), "(" | ")" | "," | "token_tree") {
                            arg_list.push(arg);
                        } else if arg.kind() == "token_tree" {
                            // Extract identifiers from token tree for better taint tracking
                            let mut inner_cursor = arg.walk();
                            for inner in arg.children(&mut inner_cursor) {
                                if inner.kind() == "identifier" {
                                    arg_list.push(inner);
                                }
                            }
                        }
                    }
                    ops.push(SemanticOp::Call {
                        function_name: &source[m.start_byte()..m.end_byte()],
                        args: arg_list,
                        node,
                    });
                }
            }
            "function_item" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    ops.push(SemanticOp::Binding {
                        name: &source[name_node.start_byte()..name_node.end_byte()],
                        value_node: node,
                    });
                }
                if let Some(body) = node.child_by_field_name("body") {
                    ops.push(SemanticOp::EnterBlock(body));
                }
            }
            _ => {}
        }
    }
}
