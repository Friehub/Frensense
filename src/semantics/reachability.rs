// SPDX-License-Identifier: MIT

use tree_sitter::Node;

/// A structural AST walker that determines if a pattern is reachable
/// within a given node's execution paths.
pub struct ReachabilityChecker<'a> {
    source: &'a str,
}

impl<'a> ReachabilityChecker<'a> {
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Returns true if at least one reachable path through `body`
    /// contains a node whose text matches `pattern`.
    #[must_use]
    pub fn any_reachable_path_contains(&self, body: Node<'a>, pattern: &regex::Regex) -> bool {
        self.walk_reachable(body, pattern, false)
    }

    fn walk_reachable(
        &self,
        node: Node<'a>,
        pattern: &regex::Regex,
        inside_dead_branch: bool,
    ) -> bool {
        if inside_dead_branch {
            return false;
        }

        let kind = node.kind();

        // Handle if statements specially to prune dead branches
        if (kind == "if_statement" || kind == "if_expression")
            && let Some(cond) = node.child_by_field_name("condition")
        {
            let (consequence_dead, alternative_dead) = match self.evaluate_condition(cond) {
                Some(true) => (false, true),
                Some(false) => (true, false),
                None => (false, false),
            };

            // Visit condition
            let cond_text = &self.source[cond.start_byte()..cond.end_byte()];
            if pattern.is_match(cond_text) {
                return true;
            }

            // Visit consequence
            if let Some(consequence) = node.child_by_field_name("consequence")
                && self.walk_reachable(consequence, pattern, consequence_dead)
            {
                return true;
            }

            // Visit alternative
            if let Some(alternative) = node.child_by_field_name("alternative")
                && self.walk_reachable(alternative, pattern, alternative_dead)
            {
                return true;
            }

            return false;
        }

        let text = &self.source[node.start_byte()..node.end_byte()];

        // We only match against non-comment and non-block nodes to avoid false positives
        // from comments or large containers that happen to contain the pattern.
        if !kind.contains("comment") && !kind.contains("block") && pattern.is_match(text) {
            return true;
        }

        let mut cursor = node.walk();
        let mut path_terminated = false;

        for child in node.children(&mut cursor) {
            if child.kind() == "comment" {
                continue;
            }
            if path_terminated {
                // Siblings after a return/throw are dead
                continue;
            }

            if self.walk_reachable(child, pattern, false) {
                return true;
            }

            // Check if this child terminates the path (unconditional return/throw)
            let c_kind = child.kind();
            if c_kind == "return_statement" || c_kind == "throw_statement" {
                path_terminated = true;
            }
        }
        false
    }

    fn evaluate_condition(&self, node: Node) -> Option<bool> {
        let code = self.source[node.start_byte()..node.end_byte()]
            .trim()
            .to_lowercase();
        match code.as_str() {
            "true" | "1" | "!false" | "!0" => Some(true),
            "false" | "0" | "!true" | "!1" => Some(false),
            _ => None,
        }
    }
}
