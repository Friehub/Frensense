// SPDX-License-Identifier: MIT

use tree_sitter::Node;

/// Iterator that walks up the tree from a node to the root.
pub struct AncestorIter<'a> {
    node: Option<Node<'a>>,
}

impl<'a> AncestorIter<'a> {
    pub fn new(node: Node<'a>) -> Self {
        Self { node: Some(node) }
    }
}

impl<'a> Iterator for AncestorIter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node?;
        self.node = current.parent();
        Some(current)
    }
}

/// Get the source text of a node.
pub fn node_text<'a>(node: Node<'a>, source: &'a str) -> &'a str {
    node.utf8_text(source.as_bytes()).unwrap_or("")
}

/// Check if a node has an ancestor with the given kind.
pub fn has_ancestor_kind(node: Node, kind: &str) -> bool {
    AncestorIter::new(node).any(|n| n.kind() == kind)
}

/// Find the first ancestor of a node matching a predicate.
pub fn find_parent_kind<F>(node: Node, predicate: F) -> Option<Node>
where
    F: Fn(Node) -> bool,
{
    AncestorIter::new(node).skip(1).find(|&n| predicate(n))
}

/// Check if a node is inside a $transaction or prisma.$transaction block.
pub fn is_inside_transaction(node: Node, source: &str) -> bool {
    find_parent_kind(node, |n| {
        if n.kind() == "call_expression" {
            let text = node_text(n, source);
            return text.contains("$transaction") || text.contains("transaction");
        }
        false
    })
    .is_some()
}

/// Collect all function/method call names in a subtree.
pub fn collect_calls_in_scope<'a>(node: Node<'a>, source: &'a str) -> Vec<(String, Node<'a>)> {
    let mut calls = Vec::new();
    collect_calls_recursive(node, source, &mut calls);
    calls
}

fn collect_calls_recursive<'a>(node: Node<'a>, source: &'a str, calls: &mut Vec<(String, Node<'a>)>) {
    if node.kind() == "call_expression" {
        if let Some(callee) = node.child_by_field_name("function") {
            let text = node_text(callee, source).to_string();
            calls.push((text, node));
        }
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_calls_recursive(child, source, calls);
        }
    }
}

/// Check if a node is a database read operation (Prisma find*, findUnique, etc.).
pub fn is_db_read(node: Node, source: &str) -> Option<String> {
    if node.kind() != "call_expression" {
        return None;
    }
    let text = node_text(node, source);
    let read_methods = [
        "findUnique", "findFirst", "findMany", "findUniqueOrThrow",
        "findFirstOrThrow", "findFirst_", "findMany_",
        "aggregate", "count", "groupBy",
    ];
    for method in &read_methods {
        if text.contains(method) {
            return Some(method.to_string());
        }
    }
    None
}

/// Check if a node is a database write operation (Prisma create, update, etc.).
pub fn is_db_write(node: Node, source: &str) -> Option<String> {
    if node.kind() != "call_expression" {
        return None;
    }
    let text = node_text(node, source);
    let write_methods = [
        "create", "createMany", "update", "updateMany",
        "upsert", "delete", "deleteMany",
        "executeRaw", "executeRawUnsafe",
    ];
    for method in &write_methods {
        if text.contains(method) {
            return Some(method.to_string());
        }
    }
    None
}

/// Check if a node is a conditional check (if statement, ternary, etc.).
pub fn is_conditional_check(node: Node) -> bool {
    matches!(
        node.kind(),
        "if_statement" | "if_expression" | "conditional_expression"
            | "switch_statement" | "match_expression"
    )
}

/// Extract variable names referenced in a node's condition.
pub fn extract_condition_refs<'a>(node: Node<'a>, source: &'a str) -> Vec<String> {
    let mut refs = Vec::new();
    extract_identifiers(node, source, &mut refs);
    refs
}

fn extract_identifiers<'a>(node: Node<'a>, source: &'a str, refs: &mut Vec<String>) {
    if node.kind() == "identifier" || node.kind() == "property_identifier" {
        let name = node_text(node, source).to_string();
        if !refs.contains(&name) {
            refs.push(name);
        }
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            extract_identifiers(child, source, refs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancestor_iter() {
        let source = "fn foo() { if true { bar(); } }";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        // Find the innermost node
        let if_stmt = root.descendant_for_point_range(
            tree_sitter::Point { row: 0, column: 10 },
            tree_sitter::Point { row: 0, column: 20 },
        );
        if let Some(node) = if_stmt {
            let ancestors: Vec<_> = AncestorIter::new(node).map(|n| n.kind()).collect();
            assert!(!ancestors.is_empty());
        }
    }

    #[test]
    fn test_is_db_read() {
        let source = "prisma.user.findUnique({ where: { id } })";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        assert!(is_db_read(root, source).is_some());
    }

    #[test]
    fn test_is_db_write() {
        let source = "prisma.user.create({ data: { name } })";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        let root = tree.root_node();
        assert!(is_db_write(root, source).is_some());
    }
}
