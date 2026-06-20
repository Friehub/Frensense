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
pub fn diff_ast(
    positive_source: &str,
    negative_source: &str,
    _positive_path: &str,
    _negative_path: &str,
) -> Result<AstDiff, String> {
    // Extract function calls from both
    let pos_calls = extract_calls(positive_source);
    let neg_calls = extract_calls(negative_source);

    // Extract function names from both
    let pos_funcs = extract_functions(positive_source);
    let neg_funcs = extract_functions(negative_source);

    // Find modified functions
    let mut modified_functions = Vec::new();
    for (name, pos_line) in &pos_funcs {
        if let Some(neg_line) = neg_funcs.get(name) {
            let pos_calls_in_func = get_calls_in_function(positive_source, name);
            let neg_calls_in_func = get_calls_in_function(negative_source, name);

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

/// Extract function calls from source code.
fn extract_calls(source: &str) -> Vec<(String, String)> {
    let mut calls = Vec::new();
    let re = regex::Regex::new(r"(\w+)\s*\(\s*(\w+)").unwrap();

    for cap in re.captures_iter(source) {
        let caller = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let callee = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
        calls.push((caller, callee));
    }

    calls
}

/// Extract function declarations from source code.
fn extract_functions(source: &str) -> HashMap<String, usize> {
    let mut funcs = HashMap::new();
    let re = regex::Regex::new(r"function\s+(\w+)").unwrap();

    for (i, line) in source.lines().enumerate() {
        if let Some(cap) = re.captures(line) {
            if let Some(name) = cap.get(1) {
                funcs.insert(name.as_str().to_string(), i + 1);
            }
        }
    }

    funcs
}

/// Get calls within a specific function.
fn get_calls_in_function(source: &str, func_name: &str) -> Vec<String> {
    let mut calls = Vec::new();
    let re = regex::Regex::new(&format!(r"function\s+{}\s*\([^)]*\)\s*\{{", regex::escape(func_name))).unwrap();

    if let Some(m) = re.find(source) {
        let start = m.end();
        // Find matching closing brace
        let mut depth = 1;
        let mut end = start;
        for (i, c) in source[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + i;
                        break;
                    }
                }
                _ => {}
            }
        }

        let body = &source[start..end];
        let call_re = regex::Regex::new(r"(\w+)\s*\(").unwrap();
        for cap in call_re.captures_iter(body) {
            if let Some(name) = cap.get(1) {
                let name = name.as_str().to_string();
                if name != func_name && name != "if" && name != "for" && name != "while" {
                    calls.push(name);
                }
            }
        }
    }

    calls
}

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
                description: format!("Added call: {}", call),
            });
        }
    }

    // Calls removed in negative (bugs removed)
    for call in &pos_set {
        if !neg_set.contains(call) {
            changes.push(AstChange {
                kind: ChangeKind::CallRemoved,
                description: format!("Removed call: {}", call),
            });
        }
    }

    changes
}

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
        let positive = r#"
function handler(req) {
    const input = req.body.query;
    eval(input);
}
"#;
        let negative = r#"
function handler(req) {
    const input = req.body.query;
    const clean = sanitize(input);
    eval(clean);
}
"#;

        let diff = diff_ast(positive, negative, "test.ts", "test.ts").unwrap();

        // Should detect that negative has a sanitizer added
        assert!(!diff.modified_functions.is_empty());
    }
}
