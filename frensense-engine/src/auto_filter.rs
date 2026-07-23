// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use crate::corpus::bundle::BundlePattern;
use crate::corpus::semantic::SemanticFilter;

/// Auto-derived filter suggestions, keyed by pattern id.
#[derive(Debug, Clone, Default)]
pub struct AutoFilterStats {
    pub contains_import: HashMap<String, Vec<String>>,
    pub contains_call_to: HashMap<String, Vec<String>>,
    /// Calls that appear in negatives but not in positives (must_not_contain).
    pub excludes_call: HashMap<String, Vec<String>>,
    /// Common function-name prefix regex patterns within a category.
    pub function_name_regex: HashMap<String, String>,
    /// AST node types that appear in negatives but not in positives.
    pub excludes_node_type: HashMap<String, Vec<String>>,
    /// Function names that appear in negatives but not in positives.
    pub excludes_function_name: HashMap<String, Vec<String>>,
}

fn extract_category(pattern_id: &str) -> &str {
    pattern_id.split('_').nth(1).unwrap_or("default")
}

const IMPORT_WITHIN_RATIO: f64 = 0.25;
const IMPORT_EXCLUSIVITY: f64 = 3.0;
const CALL_WITHIN_RATIO: f64 = 0.25;
const CALL_EXCLUSIVITY: f64 = 3.0;
const MIN_POSITIVES: usize = 5;

pub fn compute_auto_filters(
    patterns: &[BundlePattern],
    source_texts: &HashMap<String, String>,
) -> AutoFilterStats {
    let total = patterns.len() as f64;
    let mut contains_import: HashMap<String, Vec<String>> = HashMap::new();
    let mut contains_call_to: HashMap<String, Vec<String>> = HashMap::new();

    // Group by category
    let mut by_cat: HashMap<&str, Vec<&BundlePattern>> = HashMap::new();
    for p in patterns {
        if !p.positives.is_empty() {
            by_cat.entry(extract_category(&p.id)).or_default().push(p);
        }
    }

    for (&cat, cat_pats) in &by_cat {
        if cat_pats.len() < MIN_POSITIVES {
            continue;
        }
        let n_cat = cat_pats.len() as f64;
        let n_non = (total - n_cat).max(1.0);

        // --- Import exclusivity ---
        let mut cat_imp: HashMap<String, usize> = HashMap::new();
        let mut non_imp: HashMap<String, usize> = HashMap::new();
        for p in patterns {
            let src = source_texts.get(&p.id).map(|s| s.as_str()).unwrap_or("");
            let imps = extract_imports(src);
            if extract_category(&p.id) == cat {
                for i in imps { *cat_imp.entry(i).or_insert(0) += 1; }
            } else {
                for i in imps { *non_imp.entry(i).or_insert(0) += 1; }
            }
        }
        for (imp, freq) in &cat_imp {
            if *freq as f64 / n_cat < IMPORT_WITHIN_RATIO { continue; }
            let nf = non_imp.get(imp).copied().unwrap_or(0) as f64 / n_non;
            if nf == 0.0 || (*freq as f64 / n_cat) / nf >= IMPORT_EXCLUSIVITY {
                for p in cat_pats {
                    contains_import.entry(p.id.clone()).or_default().push(imp.clone());
                }
            }
        }

        // --- Call target exclusivity ---
        let mut cat_call: HashMap<String, usize> = HashMap::new();
        let mut non_call: HashMap<String, usize> = HashMap::new();
        for p in patterns {
            let src = source_texts.get(&p.id).map(|s| s.as_str()).unwrap_or("");
            let calls = extract_call_targets(src);
            if extract_category(&p.id) == cat {
                for c in calls { *cat_call.entry(c).or_insert(0) += 1; }
            } else {
                for c in calls { *non_call.entry(c).or_insert(0) += 1; }
            }
        }
        // Also incorporate api_calls from fingerprints
        for p in patterns {
            if extract_category(&p.id) == cat {
                for fp in &p.positives {
                    for &h in &fp.api_calls {
                        // Can't reverse hash, rely on source-text extraction above
                        let _ = h;
                    }
                }
            }
        }
        for (call, freq) in &cat_call {
            if *freq as f64 / n_cat < CALL_WITHIN_RATIO { continue; }
            let nf = non_call.get(call).copied().unwrap_or(0) as f64 / n_non;
            if nf == 0.0 || (*freq as f64 / n_cat) / nf >= CALL_EXCLUSIVITY {
                for p in cat_pats {
                    contains_call_to.entry(p.id.clone()).or_default().push(call.clone());
                }
            }
        }
    }

    // === Per-pattern negative-exclusivity learning ===
    // For each pattern, identify calls and node types that appear in ALL
    // negatives but NOT in any positive — these become must_not_contain constraints.
    let mut excludes_call: HashMap<String, Vec<String>> = HashMap::new();
    let mut excludes_node_type: HashMap<String, Vec<String>> = HashMap::new();
    let mut excludes_function_name: HashMap<String, Vec<String>> = HashMap::new();
    let mut function_name_regex: HashMap<String, String> = HashMap::new();

    for p in patterns {
        if p.positives.is_empty() || p.negatives.is_empty() {
            continue;
        }
        let src_pos = source_texts.get(&p.id).map(|s| s.as_str()).unwrap_or("");
        let src_neg = get_negative_source(&p.id, source_texts);

        // --- Excludes call ---
        let pos_calls: std::collections::HashSet<String> =
            extract_call_targets(src_pos).into_iter().collect();
        let neg_calls: std::collections::HashSet<String> =
            extract_call_targets(&src_neg).into_iter().collect();
        // Calls in ALL negatives but NO positives
        let excludes: Vec<String> = neg_calls.difference(&pos_calls).cloned().collect();
        if !excludes.is_empty() {
            excludes_call.insert(p.id.clone(), excludes);
        }

        // --- Excludes node type ---
        let pos_nodes: std::collections::HashSet<String> =
            extract_node_types(src_pos).into_iter().collect();
        let neg_nodes: std::collections::HashSet<String> =
            extract_node_types(&src_neg).into_iter().collect();
        let excl_nodes: Vec<String> = neg_nodes.difference(&pos_nodes).cloned().collect();
        if !excl_nodes.is_empty() {
            excludes_node_type.insert(p.id.clone(), excl_nodes);
        }

        // --- Excludes function name ---
        let pos_fnames: std::collections::HashSet<String> = p.positives
            .iter().map(|fp| fp.function_name.clone()).collect();
        let neg_fnames: std::collections::HashSet<String> = p.negatives
            .iter().map(|fp| fp.function_name.clone()).collect();
        let excl_fnames: Vec<String> = neg_fnames.difference(&pos_fnames).cloned().collect();
        if !excl_fnames.is_empty() {
            excludes_function_name.insert(p.id.clone(), excl_fnames);
        }

        // --- Function name regex ---
        // If all positives share the same function name prefix, learn it.
        let pos_fnames_vec: Vec<&str> = p.positives.iter().map(|fp| fp.function_name.as_str()).collect();
        if let Some(prefix) = common_prefix(&pos_fnames_vec) {
            function_name_regex.insert(p.id.clone(), format!("^{prefix}"));
        }
    }

    AutoFilterStats {
        contains_import,
        contains_call_to,
        excludes_call,
        function_name_regex,
        excludes_node_type,
        excludes_function_name,
    }
}

pub fn merge_filters(
    hand: &SemanticFilter,
    auto: Option<&AutoFilterStats>,
    pid: &str,
) -> SemanticFilter {
    let Some(auto) = auto else { return hand.clone(); };
    let mut m = hand.clone();

    if let Some(imps) = auto.contains_import.get(pid) {
        m.contains_import.extend(imps.iter().cloned());
    }
    if let Some(calls) = auto.contains_call_to.get(pid) {
        m.contains_call_to.extend(calls.iter().cloned());
    }
    // Auto-learned must_not_contain constraints — DISABLED pending
    // frequency-threshold tuning to avoid over-exclusion.
    // See https://github.com/Friehub/Frensense/issues/auto-filter-excludes
    // if let Some(excl) = auto.excludes_call.get(pid) {
    //     m.must_not_contain_call_to.extend(excl.iter().cloned());
    // }
    // if let Some(re) = auto.function_name_regex.get(pid) {
    //     if m.function_name_regex.is_none() {
    //         m.function_name_regex = Some(re.clone());
    //     }
    // }
    // if let Some(nodes) = auto.excludes_node_type.get(pid) {
    //     m.must_not_contain_node_type.extend(nodes.iter().cloned());
    // }
    // if let Some(fnames) = auto.excludes_function_name.get(pid) {
    //     m.must_not_match_function_name.extend(fnames.iter().cloned());
    // }
    m
}

fn extract_imports(source: &str) -> Vec<String> {
    let mut r = Vec::new();
    for line in source.lines() {
        let t = line.trim();
        if let Some(idx) = t.find("from ") {
            let after = t[idx + 5..].trim();
            let pkg = after.trim_start_matches('\'').trim_start_matches('"');
            let pkg = pkg.trim_end_matches('\'').trim_end_matches('"').trim_end_matches(';');
            if !pkg.is_empty() && !pkg.contains(' ') {
                r.push(pkg.to_string());
            }
        }
        if let Some(s) = t.find("require(") {
            let inner = t[s + 8..].trim();
            if let Some(e) = inner.find(')') {
                let pkg = inner[..e].trim().trim_matches('\'').trim_matches('"');
                if !pkg.is_empty() {
                    r.push(pkg.to_string());
                }
            }
        }
    }
    r
}

/// Get source text for a pattern's negative variant (try _negative.ts, _negative2.ts, etc.)
fn get_negative_source(pattern_id: &str, sources: &HashMap<String, String>) -> String {
    // Try exact match first (negative file content may be keyed under pattern_id)
    if let Some(src) = sources.get(pattern_id) {
        return src.clone();
    }
    String::new()
}

/// Extract AST node type names from source text (crude heuristic: match keywords).
fn extract_node_types(source: &str) -> Vec<String> {
    let mut r = Vec::new();
    let keywords = [
        "return", "if", "else", "for", "while", "switch", "try", "catch",
        "throw", "await", "async", "new", "delete", "typeof", "instanceof",
        "import", "export", "class", "function", "const", "let", "var",
    ];
    let tokens: Vec<&str> = source.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|t| !t.is_empty())
        .collect();
    for &kw in &keywords {
        if tokens.contains(&kw) {
            r.push(kw.to_string());
        }
    }
    r
}

/// Find the longest common prefix among a set of strings.
fn common_prefix<'a>(names: &[&'a str]) -> Option<String> {
    if names.is_empty() { return None; }
    if names.len() == 1 { return Some(names[0].to_string()); }
    let first = names[0].as_bytes();
    for len in (1..=first.len()).rev() {
        let prefix = &first[..len];
        if names[1..].iter().all(|n| n.as_bytes().starts_with(prefix)) {
            return Some(String::from_utf8_lossy(prefix).to_string());
        }
    }
    None
}

fn extract_call_targets(source: &str) -> Vec<String> {
    let mut r = Vec::new();
    for line in source.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with('*') { continue; }
        let mut buf = String::new();
        for ch in t.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '$' {
                buf.push(ch);
            } else if ch == '(' && !buf.is_empty() {
                let name = buf.rsplit('.').next().unwrap_or(&buf).to_string();
                r.push(name);
                buf.clear();
            } else {
                buf.clear();
            }
        }
    }
    r
}


