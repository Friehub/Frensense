use crate::corpus::semantic::SemanticFilter;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
