use crate::corpus::semantic::SemanticFilter;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static CALL_TARGET_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"([a-zA-Z0-9_]+)\s*\(").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoFilterStats {
    pub contains_call_to: HashMap<String, Vec<String>>,
    pub must_not_contain_call_to: HashMap<String, Vec<String>>,
    pub function_name_regex: HashMap<String, String>,
    pub contains_node_type: HashMap<String, Vec<String>>,
    pub must_not_contain_node_type: HashMap<String, Vec<String>>,
    pub must_not_match_function_name: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoFilterEntry {
    pub pattern_id: String,
    pub required_calls: HashSet<String>,
    pub forbidden_calls: HashSet<String>,
    pub required_node_types: HashSet<String>,
    pub forbidden_node_types: HashSet<String>,
    pub forbidden_fn_names: HashSet<String>,
}

pub fn merge_filters(
    manual: Option<&SemanticFilter>,
    auto: Option<&AutoFilterStats>,
    pattern_id: &str,
) -> SemanticFilter {
    let mut merged = manual.cloned().unwrap_or_default();

    if let Some(stats) = auto {
        if let Some(calls) = stats.contains_call_to.get(pattern_id) {
            for call in calls {
                if !merged.contains_call_to.contains(call) {
                    merged.contains_call_to.push(call.clone());
                }
            }
        }
        if let Some(excludes) = stats.must_not_contain_call_to.get(pattern_id) {
            for ex in excludes {
                if !merged.must_not_contain_call_to.contains(ex) {
                    merged.must_not_contain_call_to.push(ex.clone());
                }
            }
        }
        if let Some(req_nodes) = stats.contains_node_type.get(pattern_id) {
            for node in req_nodes {
                if !merged.contains_node_type.contains(node) {
                    merged.contains_node_type.push(node.clone());
                }
            }
        }
        if let Some(excludes) = stats.must_not_contain_node_type.get(pattern_id) {
            for ex in excludes {
                if !merged.must_not_contain_node_type.contains(ex) {
                    merged.must_not_contain_node_type.push(ex.clone());
                }
            }
        }
        if let Some(re) = stats.function_name_regex.get(pattern_id) {
            if merged.function_name_regex.is_none() {
                merged.function_name_regex = Some(re.clone());
            }
        }
        if let Some(fnames) = stats.must_not_match_function_name.get(pattern_id) {
            for fname in fnames {
                if !merged.must_not_match_function_name.contains(fname) {
                    merged.must_not_match_function_name.push(fname.clone());
                }
            }
        }
    }
    merged
}

/// Strip comments and string literals from source text for safer regex matching.
fn strip_comments_and_strings(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut in_block = false;
    let mut in_line = false;
    let mut in_str = false;
    let mut str_char = '\0';

    while let Some(c) = chars.next() {
        if in_block {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block = false;
            }
            continue;
        }
        if in_line {
            if c == '\n' {
                in_line = false;
                out.push(c);
            }
            continue;
        }
        if in_str {
            if c == '\\' {
                chars.next();
            } else if c == str_char {
                in_str = false;
            }
            continue;
        }

        if c == '/' {
            if chars.peek() == Some(&'*') {
                chars.next();
                in_block = true;
                continue;
            } else if chars.peek() == Some(&'/') {
                chars.next();
                in_line = true;
                continue;
            }
        } else if c == '"' || c == '\'' || c == '`' {
            in_str = true;
            str_char = c;
            continue;
        }

        out.push(c);
    }
    out
}

/// Extract call targets from source text using a pre-compiled regex.
/// Returns short names only (e.g., "runInContext" not "vm.runInContext").
pub fn extract_call_targets(source: &str) -> HashSet<String> {
    let clean_source = strip_comments_and_strings(source);
    let mut targets = HashSet::new();
    for cap in CALL_TARGET_RE.captures_iter(&clean_source) {
        targets.insert(cap[1].to_string());
    }
    targets
}
