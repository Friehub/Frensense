// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use crate::corpus::bundle::BundlePattern;
use crate::corpus::semantic::SemanticFilter;

/// Auto-derived filter suggestions, keyed by pattern id.
#[derive(Debug, Clone, Default)]
pub struct AutoFilterStats {
    pub contains_import: HashMap<String, Vec<String>>,
    pub contains_call_to: HashMap<String, Vec<String>>,
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

    AutoFilterStats { contains_import, contains_call_to }
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


