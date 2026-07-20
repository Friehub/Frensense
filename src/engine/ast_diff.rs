// SPDX-License-Identifier: MIT

//! AST diffing for pattern learning.
//!
//! Compares two source files and extracts what changed.

use std::collections::{HashMap, HashSet};

/// Result of diffing two ASTs.
#[derive(Debug, Clone)]
pub struct AstDiff {
    /// Functions that differ between positive and negative
    pub modified_functions: Vec<FunctionDiff>,
    /// Call graph differences
    pub call_diffs: Vec<CallDiff>,
}

#[derive(Debug, Clone)]
pub struct FunctionDiff {
    pub name: String,
    pub positive_line: usize,
    pub negative_line: usize,
    pub changes: Vec<AstChange>,
}

#[derive(Debug, Clone)]
pub struct AstChange {
    pub kind: ChangeKind,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeKind {
    /// A function call was added
    CallAdded,
    /// A function call was removed
    CallRemoved,
    /// A variable assignment changed
    AssignmentChanged,
    /// A conditional was added/removed
    ConditionalChanged,
}

impl std::fmt::Display for ChangeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CallAdded => write!(f, "Call Added"),
            Self::CallRemoved => write!(f, "Call Removed"),
            Self::AssignmentChanged => write!(f, "Assignment Changed"),
            Self::ConditionalChanged => write!(f, "Conditional Changed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallDiff {
    pub caller: String,
    pub callee_positive: Option<String>,
    pub callee_negative: Option<String>,
}

/// Diff two source files at the AST level.
///
/// # Errors
/// Returns an error if the ASTs cannot be parsed or analyzed.
pub fn diff_ast(
    positive_source: &str,
    negative_source: &str,
    positive_path: &str,
    negative_path: &str,
) -> Result<AstDiff, String> {
    let pos_lang = frensense_engine::parser::ParserRegistry::get_language(std::path::Path::new(positive_path)).map_err(|e| e.to_string())?;
    let neg_lang = frensense_engine::parser::ParserRegistry::get_language(std::path::Path::new(negative_path)).map_err(|e| e.to_string())?;

    let mut pos_parser = tree_sitter::Parser::new();
    pos_parser.set_language(&pos_lang).map_err(|e| e.to_string())?;
    let pos_tree = pos_parser.parse(positive_source, None).ok_or("Failed to parse positive source")?;

    let mut neg_parser = tree_sitter::Parser::new();
    neg_parser.set_language(&neg_lang).map_err(|e| e.to_string())?;
    let neg_tree = neg_parser.parse(negative_source, None).ok_or("Failed to parse negative source")?;

    let pos_root = pos_tree.root_node();
    let neg_root = neg_tree.root_node();

    // Extract function calls from both
    let pos_calls = extract_calls(pos_root, positive_source);
    let neg_calls = extract_calls(neg_root, negative_source);

    // Extract function names from both
    let pos_funcs = extract_functions(pos_root, positive_source);
    let neg_funcs = extract_functions(neg_root, negative_source);

    // Find modified functions
    let mut modified_functions = Vec::new();
    for (name, pos_line) in &pos_funcs {
        if let Some(neg_line) = neg_funcs.get(name) {
            let pos_calls_in_func = get_calls_in_function(pos_root, positive_source, name);
            let neg_calls_in_func = get_calls_in_function(neg_root, negative_source, name);

            let changes = analyze_changes(&pos_calls_in_func, &neg_calls_in_func);
            if !changes.is_empty() {
                modified_functions.push(FunctionDiff {
                    name: name.clone(),
                    positive_line: *pos_line,
                    negative_line: *neg_line,
                    changes,
                });
            }
        }
    }

    // Find call graph differences
    let call_diffs = analyze_call_diffs(&pos_calls, &neg_calls);

    Ok(AstDiff {
        modified_functions,
        call_diffs,
    })
}

/// Extract function calls from AST node.
fn extract_calls(root: tree_sitter::Node, source: &str) -> Vec<(String, String)> {
    let mut calls = Vec::new();
    
    fn walk(node: tree_sitter::Node, source: &str, calls: &mut Vec<(String, String)>) {
        let kind = node.kind();
        if kind == "call_expression" || kind == "call" {
            let func_node = node.child_by_field_name("function").or_else(|| node.child(0));
            let args_node = node.child_by_field_name("arguments");
            
            if let (Some(func), Some(args)) = (func_node, args_node) {
                let func_name = source[func.start_byte()..func.end_byte()].to_string();
                let short_func = func_name.split('.').last().unwrap_or(&func_name).trim();
                
                let mut first_arg = String::new();
                for i in 0..args.child_count() {
                    let arg = args.child(i).unwrap();
                    if !matches!(arg.kind(), "(" | ")" | ",") {
                        first_arg = source[arg.start_byte()..arg.end_byte()].to_string();
                        if let Some(first_word) = first_arg.split(|c: char| !c.is_alphanumeric() && c != '_').find(|s| !s.is_empty()) {
                            first_arg = first_word.to_string();
                        }
                        break;
                    }
                }
                
                if !short_func.is_empty() && !first_arg.is_empty() {
                    calls.push((short_func.to_string(), first_arg));
                }
            }
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk(child, source, calls);
        }
    }
    
    walk(root, source, &mut calls);
    calls
}

/// Extract function declarations from AST.
fn extract_functions(root: tree_sitter::Node, source: &str) -> HashMap<String, usize> {
    let mut funcs = HashMap::new();
    
    fn walk(node: tree_sitter::Node, source: &str, funcs: &mut HashMap<String, usize>) {
        let kind = node.kind();
        if matches!(
            kind,
            "function_declaration" | "function_item" | "method_definition" | "function_definition"
        ) {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                funcs.insert(name, node.start_position().row + 1);
            }
        } else if kind == "variable_declarator" || kind == "assignment" {
            // For JS/TS arrow functions, or Python assignments
            let name_node = node.child_by_field_name("name").or_else(|| node.child_by_field_name("left"));
            let value_node = node.child_by_field_name("value").or_else(|| node.child_by_field_name("right"));
            if let (Some(n), Some(v)) = (name_node, value_node) {
                if matches!(v.kind(), "arrow_function" | "function") {
                    let name = source[n.start_byte()..n.end_byte()].to_string();
                    funcs.insert(name, node.start_position().row + 1);
                }
            }
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk(child, source, funcs);
        }
    }
    
    walk(root, source, &mut funcs);
    funcs
}

/// Get calls within a specific function.
fn get_calls_in_function(root: tree_sitter::Node, source: &str, target_func_name: &str) -> Vec<String> {
    let mut calls = Vec::new();
    let mut func_node = None;
    
    fn find_func<'a>(node: tree_sitter::Node<'a>, source: &str, target: &str, found: &mut Option<tree_sitter::Node<'a>>) {
        if found.is_some() { return; }
        let kind = node.kind();
        if matches!(kind, "function_declaration" | "function_item" | "method_definition" | "function_definition") {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = source[name_node.start_byte()..name_node.end_byte()].to_string();
                if name == target {
                    *found = Some(node);
                    return;
                }
            }
        } else if kind == "variable_declarator" || kind == "assignment" {
            let name_node = node.child_by_field_name("name").or_else(|| node.child_by_field_name("left"));
            let value_node = node.child_by_field_name("value").or_else(|| node.child_by_field_name("right"));
            if let (Some(n), Some(v)) = (name_node, value_node) {
                if matches!(v.kind(), "arrow_function" | "function") {
                    let name = source[n.start_byte()..n.end_byte()].to_string();
                    if name == target {
                        *found = Some(v);
                        return;
                    }
                }
            }
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            find_func(child, source, target, found);
        }
    }
    
    find_func(root, source, target_func_name, &mut func_node);
    
    if let Some(node) = func_node {
        fn collect_calls(node: tree_sitter::Node, source: &str, calls: &mut Vec<String>) {
            let kind = node.kind();
            if kind == "call_expression" || kind == "call" {
                if let Some(func) = node.child_by_field_name("function").or_else(|| node.child(0)) {
                    let func_name = source[func.start_byte()..func.end_byte()].to_string();
                    let short_func = func_name.split('.').last().unwrap_or(&func_name).trim();
                    if !short_func.is_empty() {
                        calls.push(short_func.to_string());
                    }
                }
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                collect_calls(child, source, calls);
            }
        }
        collect_calls(node, source, &mut calls);
    }
    
    calls
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Analyze changes between two sets of calls.
fn analyze_changes(pos_calls: &[String], neg_calls: &[String]) -> Vec<AstChange> {
    let mut changes = Vec::new();
    let pos_set: HashSet<_> = pos_calls.iter().collect();
    let neg_set: HashSet<_> = neg_calls.iter().collect();

    // Calls added in negative (sanitizers, fixes)
    for call in &neg_set {
        if !pos_set.contains(call) {
            changes.push(AstChange {
                kind: ChangeKind::CallAdded,
                description: format!("Added call: {call}"),
            });
        }
    }

    // Calls removed in negative (bugs removed)
    for call in &pos_set {
        if !neg_set.contains(call) {
            changes.push(AstChange {
                kind: ChangeKind::CallRemoved,
                description: format!("Removed call: {call}"),
            });
        }
    }

    changes
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Analyze call graph differences.
fn analyze_call_diffs(
    pos_calls: &[(String, String)],
    neg_calls: &[(String, String)],
) -> Vec<CallDiff> {
    let mut diffs = Vec::new();
    let pos_map: HashMap<_, _> = pos_calls.iter().cloned().collect();
    let neg_map: HashMap<_, _> = neg_calls.iter().cloned().collect();

    for (caller, pos_callee) in &pos_map {
        let neg_callee = neg_map.get(caller);
        if neg_callee != Some(pos_callee) {
            diffs.push(CallDiff {
                caller: caller.clone(),
                callee_positive: Some(pos_callee.clone()),
                callee_negative: neg_callee.cloned(),
            });
        }
    }

    diffs
}

/// Extract learned patterns from AST diff.
#[must_use]
pub fn extract_patterns_from_diff(diff: &AstDiff) -> Vec<LearnedPattern> {
    let mut patterns = Vec::new();

    for func_diff in &diff.modified_functions {
        for change in &func_diff.changes {
            match change.kind {
                ChangeKind::CallAdded => {
                    patterns.push(LearnedPattern {
                        kind: PatternKind::Sanitizer,
                        function: func_diff.name.clone(),
                        description: change.description.clone(),
                    });
                }
                ChangeKind::CallRemoved => {
                    patterns.push(LearnedPattern {
                        kind: PatternKind::BugPattern,
                        function: func_diff.name.clone(),
                        description: change.description.clone(),
                    });
                }
                _ => {}
            }
        }
    }

    patterns
}

#[derive(Debug, Clone)]
pub struct LearnedPattern {
    pub kind: PatternKind,
    pub function: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    BugPattern,
    Sanitizer,
    CallChange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_ast_basic() {
        let positive = r"
function handler(req) {
    const input = req.body.query;
    eval(input);
}
";
        let negative = r"
function handler(req) {
    const input = req.body.query;
    const clean = sanitize(input);
    eval(clean);
}
";

        let diff = diff_ast(positive, negative, "test.ts", "test.ts").unwrap();

        // Should detect that negative has a sanitizer added
        assert!(!diff.modified_functions.is_empty());
    }

    #[test]
    fn test_diff_ast_rust() {
        let positive = r#"
fn handler(req: Request) {
    let input = req.query();
    db::query(input);
}
"#;
        let negative = r#"
fn handler(req: Request) {
    let input = req.query();
    let clean = sanitize(input);
    db::query(clean);
}
"#;

        let diff = diff_ast(positive, negative, "test.rs", "test.rs").unwrap();
        assert!(!diff.modified_functions.is_empty());
        let changes = &diff.modified_functions[0].changes;
        assert!(changes.iter().any(|c| c.kind == ChangeKind::CallAdded && c.description.contains("sanitize")));
    }
}
