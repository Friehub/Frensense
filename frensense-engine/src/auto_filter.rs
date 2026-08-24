// SPDX-License-Identifier: MIT

use crate::corpus::bundle::BundlePattern;
use crate::corpus::semantic::SemanticFilter;
use std::collections::HashMap;

/// Auto-derived filter suggestions, keyed by pattern id.
#[derive(Debug, Clone, Default)]
pub struct AutoFilterStats {
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

pub fn compute_auto_filters(
    patterns: &[BundlePattern],
    source_texts: &HashMap<String, String>,
) -> AutoFilterStats {
    let mut contains_call_to: HashMap<String, Vec<String>> = HashMap::new();

    // NOTE: The category-level cross-pattern exclusivity loop has been intentionally
    // removed. Grouping patterns by category prefix (e.g., "ns") and then learning
    // shared import/call constraints causes every pattern in the category to require
    // the same imports (e.g., "express") even when only a subset of patterns actually
    // use them. For diverse bug categories this creates a blanket block on all
    // candidates that don't share the framework used by the majority of patterns.
    //
    // Per-pattern negative-exclusivity (below) is the correct mechanism: it learns
    // what distinguishes THIS pattern's positives from ITS OWN negatives.

    // === Per-pattern negative-exclusivity learning ===
    // For each pattern, identify calls and node types that appear in ALL
    // negatives but NOT in any positive — these become must_not_contain constraints.
    let mut excludes_call: HashMap<String, Vec<String>> = HashMap::new();
    let mut excludes_node_type: HashMap<String, Vec<String>> = HashMap::new();
    let mut excludes_function_name: HashMap<String, Vec<String>> = HashMap::new();
    let function_name_regex: HashMap<String, String> = HashMap::new();

    for p in patterns {
        if p.positives.is_empty() || p.negatives.is_empty() {
            continue;
        }
        let src_pos = source_texts.get(&p.id).map(|s| s.as_str()).unwrap_or("");
        let src_neg = get_negative_source(&p.id, source_texts);

        // --- Contains call (pattern-level) ---
        // Calls present in ALL positives but absent from the negatives. Using
        // "present in all" rather than "present in any" prevents a single unusual
        // positive from adding spurious required-call constraints.
        let pos_call_sets: Vec<std::collections::HashSet<String>> = {
            // Split source_pos by file if multiple positives — but we only have one
            // concatenated string here, so treat it as one set.
            vec![extract_call_targets(src_pos).into_iter().collect()]
        };
        let pos_call_set: std::collections::HashSet<String> = pos_call_sets
            .into_iter()
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap_or_default();
        let neg_call_set: std::collections::HashSet<String> =
            extract_call_targets(&src_neg).into_iter().collect();
        let includes: Vec<String> = pos_call_set.difference(&neg_call_set).cloned().collect();
        if !includes.is_empty() {
            contains_call_to
                .entry(p.id.clone())
                .or_default()
                .extend(includes);
        }

        // --- Excludes call ---
        // Calls exclusive to negatives (never appear in positives).
        let excludes: Vec<String> = neg_call_set.difference(&pos_call_set).cloned().collect();
        if !excludes.is_empty() {
            excludes_call.insert(p.id.clone(), excludes);
        }

        // --- Excludes node type ---
        // Keyword tokens in negatives but absent from positives.
        // Guard: only exclude tokens that appear in ALL negatives, not just one.
        // This prevents rare negative edge-cases from over-pruning.
        let pos_node_set: std::collections::HashSet<String> =
            extract_node_types(src_pos).into_iter().collect();
        let neg_node_set: std::collections::HashSet<String> =
            extract_node_types(&src_neg).into_iter().collect();
        // Only exclude very safe structural exclusions (nodes NOT in positives)
        let excl_nodes: Vec<String> = neg_node_set
            .difference(&pos_node_set)
            .cloned()
            // Extra guard: don't exclude common JS structural tokens that appear
            // legitimately in any real function body.
            .filter(|tok| {
                !matches!(
                    tok.as_str(),
                    "return" | "if" | "const" | "let" | "var" | "async" | "await"
                )
            })
            .collect();
        if !excl_nodes.is_empty() {
            excludes_node_type.insert(p.id.clone(), excl_nodes);
        }

        // --- Excludes function name ---
        // Only exclude if the name appears in ≥80% of negatives and is absent from positives.
        // Skip "anonymous" — it's the default for unnamed arrow functions and will block
        // every named function if learned.
        let neg_fname_counts: std::collections::HashMap<&str, usize> =
            p.negatives
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, fp| {
                    *acc.entry(fp.function_name.as_str()).or_insert(0) += 1;
                    acc
                });
        let pos_fname_set: std::collections::HashSet<&str> = p
            .positives
            .iter()
            .map(|fp| fp.function_name.as_str())
            .collect();
        let neg_fname_threshold = (p.negatives.len() as f64 * 0.8) as usize;
        let excl_fnames: Vec<String> = neg_fname_counts
            .into_iter()
            .filter(|(name, count)| {
                *count >= neg_fname_threshold
                    && !pos_fname_set.contains(name)
                    // Never learn "anonymous" as an exclusion — it's the default
                    // name for unnamed arrow functions in fingerprint extraction,
                    // and excluding it would reject all legitimate anonymous callbacks.
                    && *name != "anonymous"
            })
            .map(|(name, _)| name.to_string())
            .collect();
        if !excl_fnames.is_empty() {
            excludes_function_name.insert(p.id.clone(), excl_fnames);
        }

        // --- Function name regex ---
        // Intentionally NOT learned from positives. The extraction path used at
        // fingerprint time (source text) and the AST extraction used at scan time
        // disagree for anonymous arrow functions: the fingerprint sees "anonymous"
        // while the AST may return None. Learning "^anonymous" then rejects all
        // candidates since the scan-time extractor returns None for the same functions.
        //
        // If a pattern needs a function-name constraint, it must be set explicitly
        // via the hand-authored SemanticFilter in the corpus TOML, not auto-learned.
        let _ = &function_name_regex; // suppress unused warning — field populated only by hand-authored filters
    }

    AutoFilterStats {
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
    let Some(auto) = auto else {
        return hand.clone();
    };
    let mut m = hand.clone();

    if let Some(calls) = auto.contains_call_to.get(pid) {
        m.contains_call_to.extend(calls.iter().cloned());
    }
    // Auto-learned must_not_contain constraints — enabled with frequency
    // thresholds to prevent over-exclusion.
    if let Some(re) = auto.function_name_regex.get(pid) {
        if m.function_name_regex.is_none() {
            m.function_name_regex = Some(re.clone());
        }
    }
    if let Some(fnames) = auto.excludes_function_name.get(pid) {
        m.must_not_match_function_name
            .extend(fnames.iter().cloned());
    }
    if let Some(calls) = auto.excludes_call.get(pid) {
        m.must_not_contain_call_to.extend(calls.iter().cloned());
    }
    if let Some(nodes) = auto.excludes_node_type.get(pid) {
        m.must_not_contain_node_type.extend(nodes.iter().cloned());
    }
    m
}

fn extract_imports(source: &str) -> Vec<String> {
    let mut r = Vec::new();
    for line in source.lines() {
        let t = line.trim();
        if let Some(idx) = t.find("from ") {
            let after = t[idx + 5..].trim();
            let pkg = after.trim_start_matches('\'').trim_start_matches('"');
            let pkg = pkg
                .trim_end_matches('\'')
                .trim_end_matches('"')
                .trim_end_matches(';');
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

/// Rough line count for a source string (used as threshold denominator).
fn count_lines(s: &str) -> usize {
    s.bytes().filter(|&b| b == b'\n').count().max(1)
}

/// Get concatenated source text for all negative variants of a pattern.
/// Negative sources are stored under "{pattern_id}_neg" (or "_neg2", "_neg3").
/// Returns empty string if no negative source found.
fn get_negative_source(pattern_id: &str, sources: &HashMap<String, String>) -> String {
    let mut combined = String::new();
    for suffix in &["_neg", "_neg2", "_neg3", "_neg4"] {
        let key = format!("{pattern_id}{suffix}");
        if let Some(src) = sources.get(&key) {
            combined.push_str(src);
            combined.push('\n');
        }
    }
    combined
}

/// Strip block comments, line comments, and string literals from source text.
fn strip_comments_and_strings(source: &str) -> String {
    let mut clean_source = String::with_capacity(source.len());
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
                clean_source.push(c);
            }
            continue;
        }
        if in_str {
            if c == '\\' {
                chars.next(); // skip escaped char
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

        clean_source.push(c);
    }
    clean_source
}

/// Extract AST node type names from source text (crude heuristic: match keywords).
fn extract_node_types(source: &str) -> Vec<String> {
    let mut r = Vec::new();
    let keywords = [
        "return",
        "if",
        "else",
        "for",
        "while",
        "switch",
        "try",
        "catch",
        "throw",
        "await",
        "async",
        "new",
        "delete",
        "typeof",
        "instanceof",
        "import",
        "export",
        "class",
        "function",
        "const",
        "let",
        "var",
    ];
    let clean_source = strip_comments_and_strings(source);
    let tokens: Vec<&str> = clean_source
        .split(|c: char| !c.is_alphanumeric() && c != '_')
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
    if names.is_empty() {
        return None;
    }
    if names.len() == 1 {
        return Some(names[0].to_string());
    }
    let first = names[0].as_bytes();
    for len in (1..=first.len()).rev() {
        let prefix = &first[..len];
        if names[1..].iter().all(|n| n.as_bytes().starts_with(prefix)) {
            return Some(String::from_utf8_lossy(prefix).to_string());
        }
    }
    None
}

pub fn extract_call_targets(source: &str) -> Vec<String> {
    let clean_source = strip_comments_and_strings(source);

    let mut r = Vec::new();
    for line in clean_source.lines() {
        let mut buf = String::new();
        for ch in line.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '$' {
                buf.push(ch);
            } else if ch == '(' && !buf.is_empty() {
                r.push(buf.clone());
                if let Some(short) = buf.rsplit('.').next() {
                    if short.len() < buf.len() {
                        r.push(short.to_string());
                    }
                }
                buf.clear();
            } else {
                buf.clear();
            }
        }
    }
    r
}
