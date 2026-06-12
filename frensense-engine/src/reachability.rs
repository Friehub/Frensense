// SPDX-License-Identifier: MIT

use tree_sitter::Node;

pub struct ReachabilityChecker<'a> {
    source: &'a str,
}

impl<'a> ReachabilityChecker<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }

    pub fn check_reachability<F>(&self, root: Node<'a>, mut predicate: F) -> bool
    where
        F: FnMut(Node<'a>) -> bool,
    {
        if predicate(root) {
            return true;
        }

        let mut cursor = root.walk();
        loop {
            if cursor.goto_first_child() {
                let child = cursor.node();
                if self.is_dead_branch(child) {
                    if !cursor.goto_next_sibling() {
                        cursor.goto_parent();
                    }
                    continue;
                }
                if predicate(child) {
                    return true;
                }
                continue;
            }

            loop {
                if cursor.goto_next_sibling() {
                    let sibling = cursor.node();
                    if self.is_dead_branch(sibling) {
                        continue;
                    }
                    if predicate(sibling) {
                        return true;
                    }
                    break;
                }
                if !cursor.goto_parent() {
                    return false;
                }
            }
        }
    }

    fn is_dead_branch(&self, node: Node<'a>) -> bool {
        if node.kind() == "if_expression" || node.kind() == "ternary_expression" {
            if let Some(condition) = node.child_by_field_name("condition") {
                let cond_text = condition.utf8_text(self.source.as_bytes()).unwrap_or("");
                return cond_text == "false" || cond_text == "0";
            }
        }
        if node.kind() == "else_clause" {
            if let Some(prev) = node.prev_sibling() {
                if prev.kind() == "if_expression" {
                    if let Some(condition) = prev.child_by_field_name("condition") {
                        let cond_text = condition.utf8_text(self.source.as_bytes()).unwrap_or("");
                        return cond_text == "true" || cond_text == "1";
                    }
                }
            }
        }
        false
    }

    pub fn search_node<F>(&self, node: Node<'a>, predicate: &F) -> bool
    where
        F: Fn(Node<'a>) -> bool,
    {
        self.check_reachability(node, predicate)
    }

    pub fn any_reachable_path_contains(&self, root: Node<'a>, re: &regex::Regex) -> bool {
        self.check_reachability(root, |n| {
            let text = n.utf8_text(self.source.as_bytes()).unwrap_or("");
            re.is_match(text)
        })
    }
}
