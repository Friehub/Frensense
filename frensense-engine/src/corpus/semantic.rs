// SPDX-License-Identifier: MIT

//! Semantic filters for corpus pattern matching.
//!
//! Fingerprint-based matching alone produces false positives because structurally
//! similar code can have different semantics. For example, `setTimeout(() => abort())`
//! looks like `.then(res => res.json())` to a fingerprint matcher, but only the latter
//! is a promise chain.
//!
//! Semantic filters check AST-level constraints before scoring, ensuring patterns
//! only match code that actually exhibits the bug they're designed to detect.

use tree_sitter::Node;

/// Semantic constraint for a corpus pattern.
#[derive(Debug, Clone, Default)]
pub struct SemanticFilter {
    /// Only match functions containing calls to these targets.
    /// E.g., `["fetch", ".then"]` for promise patterns.
    pub contains_call_to: Vec<String>,

    /// Only match functions whose name matches this regex pattern.
    /// E.g., `^sanitize` for sanitizer passthrough patterns.
    pub function_name_regex: Option<String>,

    /// Only match functions that do NOT contain calls to these targets.
    /// E.g., `[".catch"]` for `promise_catch` (must NOT have .catch).
    pub must_not_contain_call_to: Vec<String>,

    /// Only match if the function body contains specific AST node types.
    /// E.g., `["await_expression"]` for async patterns.
    pub contains_node_type: Vec<String>,

    /// Only match if the function does NOT contain these node types.
    pub must_not_contain_node_type: Vec<String>,
}

impl SemanticFilter {
    /// Returns true if this filter is empty (no constraints).
    pub fn is_empty(&self) -> bool {
        self.contains_call_to.is_empty()
            && self.function_name_regex.is_none()
            && self.must_not_contain_call_to.is_empty()
            && self.contains_node_type.is_empty()
            && self.must_not_contain_node_type.is_empty()
    }

    /// Check if a candidate function matches all semantic constraints.
    pub fn matches(&self, func_node: Node<'_>, source: &str) -> bool {
        if self.is_empty() {
            return true;
        }

        // Check function name regex
        if let Some(ref pattern) = self.function_name_regex {
            if let Some(name) = extract_function_name(func_node, source) {
                if !regex_match(&name, pattern) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Collect all call targets in the function
        let calls = collect_call_targets(func_node, source);

        // Check contains_call_to
        if !self.contains_call_to.is_empty() {
            let has_match = self
                .contains_call_to
                .iter()
                .any(|target| calls.iter().any(|call| call.contains(target.as_str())));
            if !has_match {
                return false;
            }
        }

        // Check must_not_contain_call_to
        if !self.must_not_contain_call_to.is_empty() {
            let has_forbidden = self
                .must_not_contain_call_to
                .iter()
                .any(|target| calls.iter().any(|call| call.contains(target.as_str())));
            if has_forbidden {
                return false;
            }
        }

        // Check contains_node_type
        if !self.contains_node_type.is_empty() {
            let node_types = collect_node_types(func_node);
            let has_match = self
                .contains_node_type
                .iter()
                .any(|nt| node_types.iter().any(|t| t == nt));
            if !has_match {
                return false;
            }
        }

        // Check must_not_contain_node_type
        if !self.must_not_contain_node_type.is_empty() {
            let node_types = collect_node_types(func_node);
            let has_forbidden = self
                .must_not_contain_node_type
                .iter()
                .any(|nt| node_types.iter().any(|t| t == nt));
            if has_forbidden {
                return false;
            }
        }

        true
    }
}

/// Extract the function/method name from a function AST node.
fn extract_function_name(node: Node<'_>, source: &str) -> Option<String> {
    // Look for name child
    for i in 0..node.child_count() {
        let child = node.child(i)?;
        match child.kind() {
            "identifier" | "field_identifier" | "property_identifier" => {
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
            "name" => {
                // TypeScript function_declaration name
                return Some(source[child.start_byte()..child.end_byte()].to_string());
            }
            _ => {}
        }
    }

    // For arrow functions, check parent
    if node.kind() == "arrow_function" || node.kind() == "function" {
        let parent = node.parent()?;
        match parent.kind() {
            "variable_declarator" => {
                let name_node = parent.child_by_field_name("name")?;
                return Some(source[name_node.start_byte()..name_node.end_byte()].to_string());
            }
            "assignment_expression" => {
                let left = parent.child_by_field_name("left")?;
                return Some(source[left.start_byte()..left.end_byte()].to_string());
            }
            _ => {}
        }
    }

    None
}

/// Collect all call targets (function names or method names) in a function.
fn collect_call_targets(node: Node<'_>, source: &str) -> Vec<String> {
    let mut calls = Vec::new();
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();

        if n.kind() == "call_expression" {
            // TypeScript/Rust tree-sitter grammars use "function" field for the callee
            if let Some(callee) = n
                .child_by_field_name("function")
                .or_else(|| n.child_by_field_name("callee"))
            {
                let target = source[callee.start_byte()..callee.end_byte()].to_string();
                calls.push(target);
            }
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return calls;
            }
        }
    }
}

/// Collect all node types in a subtree.
fn collect_node_types(node: Node<'_>) -> Vec<String> {
    let mut types = Vec::new();
    let mut cursor = node.walk();

    loop {
        let n = cursor.node();
        types.push(n.kind().to_string());

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return types;
            }
        }
    }
}

/// Learned semantic constraints from positive/negative example pairs.
///
/// Instead of manually writing TOML filters, we automatically extract
/// what makes positives different from negatives by comparing their
/// AST features (call targets, node types).
#[derive(Debug, Clone, Default)]
pub struct LearnedConstraints {
    /// Call targets that appear in ALL positives but NOT in the corresponding negatives.
    /// These become `contains_call_to` constraints.
    pub required_calls: Vec<String>,
    /// Call targets that appear in ALL negatives but NOT in the positives.
    /// These become `must_not_contain_call_to` constraints.
    pub forbidden_calls: Vec<String>,
    /// Node types that appear in ALL positives but NOT in negatives.
    pub required_node_types: Vec<String>,
    /// Node types that appear in ALL negatives but NOT in positives.
    pub forbidden_node_types: Vec<String>,
}

impl LearnedConstraints {
    pub fn is_empty(&self) -> bool {
        self.required_calls.is_empty()
            && self.forbidden_calls.is_empty()
            && self.required_node_types.is_empty()
            && self.forbidden_node_types.is_empty()
    }

    /// Convert to a `SemanticFilter` for use in pattern matching.
    pub fn to_filter(&self) -> SemanticFilter {
        SemanticFilter {
            contains_call_to: self.required_calls.clone(),
            must_not_contain_call_to: self.forbidden_calls.clone(),
            contains_node_type: self.required_node_types.clone(),
            must_not_contain_node_type: self.forbidden_node_types.clone(),
            function_name_regex: None,
        }
    }
}

/// Learn semantic constraints by comparing positive and negative examples.
///
/// For each feature (call targets, node types):
/// - If it appears in ALL positives but NOT in ANY negative → required
/// - If it appears in ALL negatives but NOT in ANY positive → forbidden
///
/// This automatically captures what makes the buggy code different from the fix.
pub fn learn_constraints(
    positive_nodes: &[(tree_sitter::Node<'_>, &str)],
    negative_nodes: &[(tree_sitter::Node<'_>, &str)],
) -> LearnedConstraints {
    if positive_nodes.is_empty() || negative_nodes.is_empty() {
        return LearnedConstraints::default();
    }

    // Collect features from all positives
    let mut pos_call_sets: Vec<Vec<String>> = Vec::new();
    let mut pos_node_sets: Vec<Vec<String>> = Vec::new();
    for (node, source) in positive_nodes {
        let mut calls: Vec<String> = collect_call_targets(*node, source);
        calls.sort();
        calls.dedup();
        pos_call_sets.push(calls);

        let mut nodes = collect_node_types(*node);
        nodes.sort();
        nodes.dedup();
        pos_node_sets.push(nodes);
    }

    // Collect features from all negatives
    let mut neg_call_sets: Vec<Vec<String>> = Vec::new();
    let mut neg_node_sets: Vec<Vec<String>> = Vec::new();
    for (node, source) in negative_nodes {
        let mut calls: Vec<String> = collect_call_targets(*node, source);
        calls.sort();
        calls.dedup();
        neg_call_sets.push(calls);

        let mut nodes = collect_node_types(*node);
        nodes.sort();
        nodes.dedup();
        neg_node_sets.push(nodes);
    }

    // Find call targets in ALL positives
    let pos_call_universe: std::collections::HashSet<&str> = pos_call_sets
        .iter()
        .flatten()
        .map(std::string::String::as_str)
        .collect();

    let required_calls: Vec<String> = pos_call_universe
        .iter()
        .filter(|call| {
            // Must be in EVERY positive
            pos_call_sets.iter().all(|set| set.iter().any(|c| c == *call))
                // Must NOT be in ANY negative
                && !neg_call_sets.iter().any(|set| set.iter().any(|c| c == *call))
        })
        .map(std::string::ToString::to_string)
        .collect();

    // Find call targets in ALL negatives (but not in positives)
    let neg_call_universe: std::collections::HashSet<&str> = neg_call_sets
        .iter()
        .flatten()
        .map(std::string::String::as_str)
        .collect();

    let forbidden_calls: Vec<String> = neg_call_universe
        .iter()
        .filter(|call| {
            neg_call_sets
                .iter()
                .all(|set| set.iter().any(|c| c == *call))
                && !pos_call_sets
                    .iter()
                    .any(|set| set.iter().any(|c| c == *call))
        })
        .map(std::string::ToString::to_string)
        .collect();

    // Same for node types
    let pos_node_universe: std::collections::HashSet<&str> = pos_node_sets
        .iter()
        .flatten()
        .map(std::string::String::as_str)
        .collect();

    let required_node_types: Vec<String> = pos_node_universe
        .iter()
        .filter(|nt| {
            pos_node_sets.iter().all(|set| set.iter().any(|t| t == *nt))
                && !neg_node_sets.iter().any(|set| set.iter().any(|t| t == *nt))
        })
        .map(std::string::ToString::to_string)
        .collect();

    let neg_node_universe: std::collections::HashSet<&str> = neg_node_sets
        .iter()
        .flatten()
        .map(std::string::String::as_str)
        .collect();

    let forbidden_node_types: Vec<String> = neg_node_universe
        .iter()
        .filter(|nt| {
            neg_node_sets.iter().all(|set| set.iter().any(|t| t == *nt))
                && !pos_node_sets.iter().any(|set| set.iter().any(|t| t == *nt))
        })
        .map(std::string::ToString::to_string)
        .collect();

    // Filter out noise: skip very common node types that don't discriminate
    let noise_nodes: std::collections::HashSet<&str> = [
        "program",
        "statement_block",
        "expression_statement",
        "return_statement",
        "if_statement",
        "variable_declaration",
        "identifier",
        "call_expression",
        "member_expression",
        "string",
        "number",
        "true",
        "false",
        "null",
        "template_string",
        "binary_expression",
        "unary_expression",
        "parenthesized_expression",
        "comma_expression",
    ]
    .iter()
    .copied()
    .collect();

    LearnedConstraints {
        required_calls,
        forbidden_calls,
        required_node_types: required_node_types
            .into_iter()
            .filter(|nt| !noise_nodes.contains(nt.as_str()))
            .collect(),
        forbidden_node_types: forbidden_node_types
            .into_iter()
            .filter(|nt| !noise_nodes.contains(nt.as_str()))
            .collect(),
    }
}

/// Simple regex match (supports ^ and $ anchors).
fn regex_match(text: &str, pattern: &str) -> bool {
    if let Some(pat) = pattern.strip_prefix('^') {
        if let Some(pat) = pat.strip_suffix('$') {
            text == pat
        } else {
            text.starts_with(pat)
        }
    } else if let Some(pat) = pattern.strip_suffix('$') {
        text.ends_with(pat)
    } else {
        text.contains(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ts(code: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        parser.parse(code, None).unwrap()
    }

    #[test]
    fn test_function_name_regex() {
        let tree = parse_ts("function sanitizeHtml(input: string) { return input; }");
        let func = tree.root_node().child(0).unwrap();

        let filter = SemanticFilter {
            function_name_regex: Some("^sanitize".to_string()),
            ..Default::default()
        };
        assert!(filter.matches(
            func,
            "function sanitizeHtml(input: string) { return input; }"
        ));

        let filter2 = SemanticFilter {
            function_name_regex: Some("^escape".to_string()),
            ..Default::default()
        };
        assert!(!filter2.matches(
            func,
            "function sanitizeHtml(input: string) { return input; }"
        ));
    }

    #[test]
    fn test_contains_call_to() {
        let tree = parse_ts("function foo() { fetch('/api').then(r => r.json()); }");
        let func = tree.root_node().child(0).unwrap();

        let filter = SemanticFilter {
            contains_call_to: vec!["fetch".to_string()],
            ..Default::default()
        };
        assert!(filter.matches(
            func,
            "function foo() { fetch('/api').then(r => r.json()); }"
        ));

        let filter2 = SemanticFilter {
            contains_call_to: vec!["axios".to_string()],
            ..Default::default()
        };
        assert!(!filter2.matches(
            func,
            "function foo() { fetch('/api').then(r => r.json()); }"
        ));
    }

    #[test]
    fn test_must_not_contain_call_to() {
        let tree = parse_ts("function foo() { fetch('/api').then(r => r.json()); }");
        let func = tree.root_node().child(0).unwrap();

        let filter = SemanticFilter {
            contains_call_to: vec!["fetch".to_string()],
            must_not_contain_call_to: vec![".catch".to_string()],
            ..Default::default()
        };
        assert!(filter.matches(
            func,
            "function foo() { fetch('/api').then(r => r.json()); }"
        ));

        let tree2 =
            parse_ts("function foo() { fetch('/api').then(r => r.json()).catch(e => {}); }");
        let func2 = tree2.root_node().child(0).unwrap();
        assert!(!filter.matches(
            func2,
            "function foo() { fetch('/api').then(r => r.json()).catch(e => {}); }"
        ));
    }
}
