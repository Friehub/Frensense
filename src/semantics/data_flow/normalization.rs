// SPDX-License-Identifier: MIT

use frensense_engine::data_flow::alias::AliasTracker;
use frensense_engine::data_flow::TaintRegistry;
use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Normalized Semantic Operations (Owned).
/// This layer decouples the raw AST from the Analysis Engine.
#[derive(Debug, Clone)]
pub enum SemanticOp {
    /// A new variable binding (e.g., const x = y, let a = 1)
    Binding { name: String, value_range: Range },
    /// An assignment to an existing variable (e.g., x = z)
    Assignment { target: String, value_range: Range },
    /// A function or method call
    Call {
        function_name: String,
        args: Vec<Range>,
        range: Range,
    },
    /// Entry into a new block (e.g., function body, if block)
    EnterBlock(Range),
}

pub struct SemanticExtractor;

impl SemanticExtractor {
    /// Extracts normalized semantic operations from a language-specific AST node.
    #[must_use]
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
            "tuple_pattern" | "array_pattern" | "object_pattern" | "struct_pattern" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    // Skip punctuations
                    if child.kind().contains("pattern")
                        || child.kind() == "identifier"
                        || child.kind() == "shorthand_field_identifier"
                    {
                        Self::extract_bindings(child, source, value_node, ops);
                    }
                }
            }
            "tuple_struct_pattern" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind().contains("pattern") || child.kind() == "identifier" {
                        Self::extract_bindings(child, source, value_node, ops);
                    }
                }
            }
            _ => {
                // If it's a leaf identifier we didn't catch
                if node.child_count() == 0 && node.kind() == "identifier" {
                    let name = source[node.start_byte()..node.end_byte()].to_string();
                    ops.push(SemanticOp::Binding {
                        name,
                        value_range: value_node.into(),
                    });
                }
            }
        }
    }

    fn extract_typescript(node: Node, source: &str, ops: &mut Vec<SemanticOp>) {
        match node.kind() {
            "lexical_declaration" | "variable_declaration" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        let name_node = child.child_by_field_name("name");
                        let value_node = child.child_by_field_name("value");
                        if let (Some(n), Some(v)) = (name_node, value_node) {
                            Self::extract_bindings(n, source, v, ops);
                            Self::extract_typescript(v, source, ops);
                        }
                    }
                }
            }
            "assignment_expression" => {
                let left = node.child_by_field_name("left");
                let right = node.child_by_field_name("right");
                if let (Some(l), Some(r)) = (left, right) {
                    ops.push(SemanticOp::Assignment {
                        target: source[l.start_byte()..l.end_byte()].to_string(),
                        value_range: r.into(),
                    });
                    Self::extract_typescript(r, source, ops);
                }
            }
            "expression_statement" => {
                if let Some(expr) = node.child(0) {
                    Self::extract_typescript(expr, source, ops);
                }
            }
            "call_expression" => {
                let function_node = node.child_by_field_name("function");
                let arguments_node = node.child_by_field_name("arguments");

                if let (Some(f), Some(a)) = (function_node, arguments_node) {
                    let mut args = Vec::new();
                    let mut cursor = a.walk();
                    for child in a.children(&mut cursor) {
                        if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                            args.push(child.into());
                        }
                    }

                    let mut fn_name = source[f.start_byte()..f.end_byte()].to_string();

                    // Handle method calls: obj.method()
                    if f.kind() == "member_expression"
                        && let Some(prop) = f.child_by_field_name("property")
                    {
                        fn_name = source[prop.start_byte()..prop.end_byte()].to_string();
                        if let Some(receiver) = f.child_by_field_name("object") {
                            args.push(receiver.into());
                        }
                    }

                    ops.push(SemanticOp::Call {
                        function_name: fn_name,
                        args,
                        range: node.into(),
                    });
                }
            }
            "function_declaration" | "arrow_function" | "method_definition" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    ops.push(SemanticOp::Binding {
                        name: source[name_node.start_byte()..name_node.end_byte()].to_string(),
                        value_range: node.into(),
                    });
                }
                if let Some(body) = node.child_by_field_name("body") {
                    ops.push(SemanticOp::EnterBlock(body.into()));
                    Self::extract_typescript(body, source, ops);
                }
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::extract_typescript(child, source, ops);
                }
            }
        }
    }

    fn extract_rust(node: Node, source: &str, ops: &mut Vec<SemanticOp>) {
        let kind = node.kind();
        match kind {
            "let_declaration" => {
                let name_node = node.child_by_field_name("pattern");
                let value_node = node.child_by_field_name("value");
                if let (Some(n), Some(v)) = (name_node, value_node) {
                    Self::extract_bindings(n, source, v, ops);
                    Self::extract_rust(v, source, ops);
                }
            }
            "assignment_expression" => {
                let left = node.child_by_field_name("left");
                let right = node.child_by_field_name("right");
                if let (Some(l), Some(r)) = (left, right) {
                    ops.push(SemanticOp::Assignment {
                        target: source[l.start_byte()..l.end_byte()].to_string(),
                        value_range: r.into(),
                    });
                    Self::extract_rust(r, source, ops);
                }
            }
            "expression_statement" => {
                if let Some(expr) = node.child(0) {
                    Self::extract_rust(expr, source, ops);
                }
            }
            "call_expression" => {
                let function_node = node.child_by_field_name("function");
                let arguments_node = node.child_by_field_name("arguments");

                if let (Some(f), Some(a)) = (function_node, arguments_node) {
                    let mut args = Vec::new();
                    let mut cursor = a.walk();
                    for child in a.children(&mut cursor) {
                        if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                            args.push(child.into());
                        }
                    }

                    let mut fn_name = source[f.start_byte()..f.end_byte()].to_string();

                    // Handle method calls: obj.method()
                    if f.kind() == "field_expression"
                        && let Some(field) = f.child_by_field_name("field")
                    {
                        fn_name = source[field.start_byte()..field.end_byte()].to_string();
                        // Optionally track the receiver as an argument for taint propagation
                        if let Some(receiver) = f.child_by_field_name("value") {
                            args.push(receiver.into());
                        }
                    }

                    ops.push(SemanticOp::Call {
                        function_name: fn_name,
                        args,
                        range: node.into(),
                    });
                }
            }
            "macro_invocation" => {
                Self::handle_rust_macro(node, source, ops);
            }
            "function_item" => {
                if let Some(name_node) = node.child_by_field_name("name") {
                    ops.push(SemanticOp::Binding {
                        name: source[name_node.start_byte()..name_node.end_byte()].to_string(),
                        value_range: node.into(),
                    });
                }
                if let Some(body) = node.child_by_field_name("body") {
                    ops.push(SemanticOp::EnterBlock(body.into()));
                    Self::extract_rust(body, source, ops);
                }
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    Self::extract_rust(child, source, ops);
                }
            }
        }
    }

    fn handle_rust_macro(node: Node, source: &str, ops: &mut Vec<SemanticOp>) {
        let macro_name_node = node.child_by_field_name("macro");
        let args_node = node.child_by_field_name("arguments").or_else(|| {
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    let child = cursor.node();
                    if child.kind() == "token_tree" {
                        return Some(child);
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            None
        });
        if let (Some(m), Some(a)) = (macro_name_node, args_node) {
            let mut arg_list = Vec::new();
            let mut cursor = a.walk();
            for arg in a.children(&mut cursor) {
                if !matches!(arg.kind(), "(" | ")" | "," | "token_tree") {
                    arg_list.push(arg.into());
                } else if arg.kind() == "token_tree" {
                    // Extract identifiers from token tree for better taint tracking
                    let mut inner_cursor = arg.walk();
                    for inner in arg.children(&mut inner_cursor) {
                        if inner.kind() == "identifier" {
                            arg_list.push(inner.into());
                        }
                    }
                }
            }
            let op = SemanticOp::Call {
                function_name: source[m.start_byte()..m.end_byte()].to_string(),
                args: arg_list,
                range: node.into(),
            };
            ops.push(op);
        }
    }

    /// Post-process extracted ops to record variable aliases in the AliasTracker.
    /// When a binding or assignment copies a tainted variable (e.g. `const id = userInput`),
    /// records the alias so taint propagates through renames.
    pub fn record_aliases(
        ops: &[SemanticOp],
        source: &str,
        registry: &TaintRegistry,
        tracker: &mut AliasTracker,
    ) {
        for op in ops {
            let (name, value_range) = match op {
                SemanticOp::Binding { name, value_range } => (name.as_str(), value_range),
                SemanticOp::Assignment { target, value_range } => (target.as_str(), value_range),
                _ => continue,
            };
            let rhs = &source[value_range.start_byte..value_range.end_byte];
            let rhs = rhs.trim();
            // Only track simple identifier-to-identifier aliases
            if rhs.contains(' ') || rhs.contains('.') || rhs.contains('(') || rhs.contains('"') || rhs.contains('\'') {
                continue;
            }
            if registry.is_tainted(rhs) {
                tracker.record_alias(name, rhs);
            }
        }
    }
}
