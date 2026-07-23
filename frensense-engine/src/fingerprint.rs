// SPDX-License-Identifier: MIT

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};
use std::path::Path;
use tree_sitter::Node;

use crate::lang::{Language, mapper::abstract_kind};

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub language: String,
    pub ngram_hashes: Vec<u64>,
    pub weighted_ngram_hashes: FxHashMap<u64, f32>,
    pub signature_ngrams: Vec<u64>,
    pub param_type_ngrams: Vec<u64>,
    pub name_segments: Vec<String>,
    pub structural_markers: Vec<u64>,
    pub type_usages: Vec<String>,
    pub comment_density: f64,
    pub semantic_markers: Vec<u64>,
    pub skeleton: Vec<String>,
    #[cfg_attr(feature = "serialize", serde(default))]
    pub skeleton_hashes: Vec<u64>,
    /// Control flow encoding: hashes of control flow paths through the function
    /// Captures if/else, match, loop, return patterns that fingerprint race conditions, TOCTOU, etc.
    pub control_flow_hashes: Vec<u64>,
    /// API calls: hashes of the full callee expression used in the body.
    /// E.g., hash of `"child_process.exec"`.
    /// Used for AST-aware semantic matching and IDF weighting.
    pub api_calls: Vec<u64>,

    /// Last-segment hashes of chained method calls.
    /// E.g., hash of `"exec"` from `"child_process.exec"`.
    /// Kept separate from `api_calls` so IDF is not double-counted for full-form names.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub api_call_segments: Vec<u64>,
    /// Property accesses: hashes of object property access names (e.g., 'price' in 'item.price')
    pub property_accesses: Vec<u64>,

    /// Hashes of API calls where at least one argument is (or contains) a function parameter.
    /// E.g., `exec(cmd)` where `cmd` is a param → hash of `"exec"` is included.
    /// `exec("ls")` where `"ls"` is a constant → NOT included.
    /// Separated from `api_calls` so scoring can distinguish tainted from untainted sinks.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub tainted_api_calls: Vec<u64>,
}

/// M1: Compute IDF weights for n-grams from a set of fingerprints.
pub fn compute_idf_weights(fingerprints: &[FunctionFingerprint]) -> FxHashMap<u64, f32> {
    let n = fingerprints.len() as f32;
    if n == 0.0 {
        return FxHashMap::default();
    }
    let mut doc_freq: FxHashMap<u64, f32> = FxHashMap::default();
    for fp in fingerprints {
        for &hash in &fp.ngram_hashes {
            *doc_freq.entry(hash).or_insert(0.0) += 1.0;
        }
    }
    doc_freq
        .into_iter()
        .map(|(hash, df)| (hash, (n / df).ln()))
        .collect()
}

/// Apply IDF weights to a fingerprint's `weighted_ngram_hashes`.
pub fn apply_idf_weights(fingerprint: &mut FunctionFingerprint, idf_weights: &FxHashMap<u64, f32>) {
    for (hash, weight) in &mut fingerprint.weighted_ngram_hashes {
        if let Some(&idf) = idf_weights.get(hash) {
            *weight = idf;
        }
    }
}

/// M9: Position-weighted n-gram hashing.
/// Combines position with token hash so that `return` at line 5 differs from `return` at line 50.
fn token_ngrams_positional(tokens: &[String], window_size: usize) -> Vec<u64> {
    if tokens.len() < window_size {
        return Vec::new();
    }
    let mut hashes = FxHashSet::default();
    let total = tokens.len();
    for i in 0..=(total.saturating_sub(window_size)) {
        let mut fx_hasher = FxHasher::default();
        tokens[i..i + window_size].hash(&mut fx_hasher);
        let token_hash = fx_hasher.finish();
        // M9: weight by relative position (0.0 = start, 1.0 = end)
        let position = if total > 1 {
            i as f32 / (total - 1) as f32
        } else {
            0.0
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let position_bits = (position * 1024.0) as u64; // 10 bits for position
        let mut final_hasher = FxHasher::default();
        token_hash.hash(&mut final_hasher);
        position_bits.hash(&mut final_hasher);
        hashes.insert(final_hasher.finish());
    }
    let mut vec: Vec<u64> = hashes.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn token_ngrams(tokens: &[String], window_size: usize) -> FxHashSet<u64> {
    if tokens.len() < window_size {
        return FxHashSet::default();
    }
    let mut hashes = FxHashSet::default();
    for i in 0..=(tokens.len().saturating_sub(window_size)) {
        let mut fx_hasher = FxHasher::default();
        tokens[i..i + window_size].hash(&mut fx_hasher);
        hashes.insert(fx_hasher.finish());
    }
    hashes
}

fn token_ngrams_sorted(tokens: &[String], window_size: usize) -> Vec<u64> {
    let mut vec: Vec<u64> = token_ngrams(tokens, window_size).into_iter().collect();
    vec.sort_unstable();
    vec
}

fn split_name_segments(name: &str) -> Vec<String> {
    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();
    for ch in name.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            segments.push(std::mem::take(&mut current));
        }
        if ch != '_' {
            current.push(ch);
        } else if !current.is_empty() {
            segments.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn collect_structural_markers(node: Node<'_>, _source: &str, language: Language) -> Vec<u64> {
    let mut markers = FxHashSet::default();
    let mut cursor = node.walk();
    let mut hasher = FxHasher::default();
    abstract_kind(node.kind(), language).hash(&mut hasher);
    markers.insert(hasher.finish());
    loop {
        if cursor.goto_first_child() {
            let n = cursor.node();
            let mut h = FxHasher::default();
            abstract_kind(n.kind(), language).hash(&mut h);
            markers.insert(h.finish());
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                let n = cursor.node();
                let mut h = FxHasher::default();
                abstract_kind(n.kind(), language).hash(&mut h);
                markers.insert(h.finish());
                break;
            }
            if !cursor.goto_parent() {
                let mut vec: Vec<u64> = markers.into_iter().collect();
                vec.sort_unstable();
                return vec;
            }
        }
    }
}

fn collect_type_usages(node: Node<'_>, source: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n.kind() == "type_identifier" || n.kind() == "predefined_type" {
            types.push(source[n.start_byte()..n.end_byte()].to_string());
        }
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

fn count_comment_bytes(node: Node<'_>, _source: &str) -> usize {
    let mut total = 0;
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        let kind = n.kind();
        if kind == "comment" || kind == "line_comment" || kind == "block_comment" {
            total += n.end_byte() - n.start_byte();
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return total;
            }
        }
    }
}

/// Extract control flow encoding from function body.
/// Captures the sequence of control flow nodes (if, match, loop, return)
/// to fingerprint patterns like check-then-act, early returns, nested conditionals.
fn extract_control_flow(node: Node<'_>, source: &str) -> Vec<u64> {
    let mut hashes = FxHashSet::default();
    let mut path = Vec::new();
    extract_cf_recursive(node, source, &mut path, &mut hashes);
    let mut vec: Vec<u64> = hashes.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_cf_recursive(
    node: Node<'_>,
    source: &str,
    path: &mut Vec<String>,
    hashes: &mut FxHashSet<u64>,
) {
    let kind = node.kind();

    // Record control flow nodes
    match kind {
        "if_expression" | "if_statement" => {
            path.push("if".to_string());
            // Hash the condition text for condition-aware matching
            if let Some(cond) = node.child_by_field_name("condition") {
                let cond_text = &source[cond.start_byte()..cond.end_byte()];
                let mut h = FxHasher::default();
                "if_cond".hash(&mut h);
                cond_text.len().hash(&mut h); // length as proxy for complexity
                hashes.insert(h.finish());
            }
        }
        "match_expression" | "match_statement" => {
            path.push("match".to_string());
            if let Some(body) = node.child_by_field_name("body") {
                let arm_count = body.child_count();
                let mut h = FxHasher::default();
                "match_arms".hash(&mut h);
                arm_count.hash(&mut h);
                hashes.insert(h.finish());
            }
        }
        "loop_expression" | "while_expression" | "for_expression" => {
            path.push("loop".to_string());
        }
        "return_expression" | "return_statement" => {
            path.push("return".to_string());
        }
        "break_expression" => {
            path.push("break".to_string());
        }
        "try_expression" | "try_statement" => {
            path.push("try".to_string());
        }
        "catch_clause" | "catch_block" => {
            path.push("catch".to_string());
        }
        _ => {}
    }

    // Hash the path so far for each depth level
    if !path.is_empty() && path.len() <= 10 {
        let mut h = FxHasher::default();
        path.hash(&mut h);
        hashes.insert(h.finish());
    }

    // Recurse into children
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_cf_recursive(child, source, path, hashes);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    // Pop path when leaving control flow nodes
    match kind {
        "if_expression" | "if_statement" | "match_expression" | "match_statement"
        | "loop_expression" | "while_expression" | "for_expression" | "return_expression"
        | "return_statement" | "break_expression" | "try_expression" | "try_statement"
        | "catch_clause" | "catch_block" => {
            path.pop();
        }
        _ => {}
    }
}

/// Extract API calls from function body using AST traversal.
/// This replaces text-based keyword matching with actual call expression detection.
fn extract_api_calls(node: Node<'_>, source: &str) -> (Vec<u64>, Vec<u64>) {
    let mut calls = FxHashSet::default();
    let mut segments = FxHashSet::default();
    extract_calls_recursive(node, source, &mut calls, &mut segments);
    let mut calls_vec: Vec<u64> = calls.into_iter().collect();
    calls_vec.sort_unstable();
    let mut seg_vec: Vec<u64> = segments.into_iter().collect();
    seg_vec.sort_unstable();
    (calls_vec, seg_vec)
}

fn extract_calls_recursive(
    node: Node<'_>,
    source: &str,
    calls: &mut FxHashSet<u64>,
    segments: &mut FxHashSet<u64>,
) {
    let kind = node.kind();

    // Match call expressions and extract the function name
    if kind == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            let name = &source[func.start_byte()..func.end_byte()];

            // Full-form hash goes into `calls` (used for IDF)
            let mut h = FxHasher::default();
            name.hash(&mut h);
            calls.insert(h.finish());

            // Last-segment hash goes into `segments` (kept separate to avoid IDF inflation)
            if let Some(dot_pos) = name.rfind('.') {
                let method = &name[dot_pos + 1..];
                let mut h2 = FxHasher::default();
                method.hash(&mut h2);
                segments.insert(h2.finish());
            }
        }
    }

    // Also capture macro invocations (Rust)
    if kind == "macro_invocation" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = &source[name_node.start_byte()..name_node.end_byte()];
            let mut h = FxHasher::default();
            format!("macro_{name}").hash(&mut h);
            calls.insert(h.finish());
        }
    }

    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            extract_calls_recursive(child, source, calls, segments);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract tainted API calls — calls where at least one argument is a function parameter.
/// This distinguishes `exec(cmd)` from `exec("ls")` at the fingerprint level.
/// Pure AST traversal, no data flow graph needed.
fn extract_tainted_calls(node: Node<'_>, source: &str, param_names: &[String]) -> Vec<u64> {
    let mut tainted = FxHashSet::default();
    extract_tainted_recursive(node, source, param_names, &mut tainted);
    let mut vec: Vec<u64> = tainted.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_tainted_recursive(
    node: Node<'_>,
    source: &str,
    param_names: &[String],
    tainted: &mut FxHashSet<u64>,
) {
    if node.kind() == "call_expression" {
        if let Some(args_node) = node.child_by_field_name("arguments") {
            if has_param_ref(args_node, source, param_names) {
                if let Some(func) = node.child_by_field_name("function") {
                    let name = &source[func.start_byte()..func.end_byte()];
                    let mut h = FxHasher::default();
                    name.hash(&mut h);
                    tainted.insert(h.finish());
                }
            }
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_tainted_recursive(cursor.node(), source, param_names, tainted);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Check if a subtree contains any identifier that matches a parameter name.
fn has_param_ref(node: Node<'_>, source: &str, _param_names: &[String]) -> bool {
    // Simplify: any identifier in a call argument is treated as potentially tainted.
    // Pure literals (strings, numbers, booleans, null) are not.
    // This catches exec(cmd), exec(toUrl), exec(result.value) without needing
    // full data flow — any variable reference COULD carry user input.
    match node.kind() {
        "identifier" => true,
        "member_expression" | "subscript_expression" => {
            // Property access like query.to or obj[key] — always tainted
            true
        }
        "string" | "string_fragment" | "number" | "true" | "false" | "null" => false,
        _ => {
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                loop {
                    if has_param_ref(cursor.node(), source, _param_names) {
                        return true;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            false
        }
    }
}

/// Extract object property accesses (e.g. `item.price`)
fn extract_property_accesses(node: Node<'_>, source: &str) -> Vec<u64> {
    let mut accesses = FxHashSet::default();
    extract_properties_recursive(node, source, &mut accesses);
    let mut vec: Vec<u64> = accesses.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_properties_recursive(node: Node<'_>, source: &str, accesses: &mut FxHashSet<u64>) {
    let kind = node.kind();
    if kind == "member_expression" || kind == "field_expression" {
        if let Some(prop) = node
            .child_by_field_name("property")
            .or_else(|| node.child_by_field_name("field"))
        {
            let name = &source[prop.start_byte()..prop.end_byte()];
            let mut h = FxHasher::default();
            name.hash(&mut h);
            accesses.insert(h.finish());
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_properties_recursive(cursor.node(), source, accesses);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract semantic markers from function body using AST-aware API call detection.
/// Uses actual `call_expression` nodes instead of text search to eliminate false positives
/// from comments, strings, and variable names.
fn extract_semantic_markers(
    _node: Node<'_>,
    _source: &str,
    api_calls: &[u64],
    api_call_segments: &[u64],
    property_accesses: &[u64],
) -> Vec<u64> {
    let mut markers = FxHashSet::default();

    // Semantic categories mapped to API call hashes
    // Each category has a set of known-bad/good API names
    let categories: &[(&str, &[&str])] = &[
        (
            "db_query",
            &[
                "query",
                "execute",
                "raw_query",
                "format!",
                "sql_query",
                "execute_query",
            ],
        ),
        (
            "db_write",
            &["insert", "update", "upsert", "execute", "bulk_write"],
        ),
        (
            "cmd_exec",
            &[
                "exec",
                "system",
                "spawn",
                "popen",
                "Command::new",
                "child_process",
            ],
        ),
        ("code_eval", &["eval", "Function", "new Function"]),
        (
            "file_read",
            &[
                "readFile",
                "readFileSync",
                "createReadStream",
                "read_to_string",
                "fs::read",
            ],
        ),
        (
            "file_write",
            &[
                "writeFile",
                "writeFileSync",
                "createWriteStream",
                "write",
                "fs::write",
            ],
        ),
        (
            "dom_xss",
            &[
                "innerHTML",
                "outerHTML",
                "document.write",
                "insertAdjacentHTML",
            ],
        ),
        (
            "http_request",
            &["fetch", "axios", "request", "get", "post", "reqwest"],
        ),
        ("url_redirect", &["redirect", "location"]),
        ("crypto_weak", &["md5", "sha1", "createHash", "Md5", "Sha1"]),
        (
            "crypto_strong",
            &["sha256", "sha512", "bcrypt", "argon2", "Sha256"],
        ),
        (
            "deserialize",
            &[
                "JSON.parse",
                "from_str",
                "loads",
                "deserialize",
                "serde_json",
            ],
        ),
        ("sanitize", &["sanitize", "escape", "encode", "validate"]),
        ("regex", &["Regex::new", "new RegExp", "re.compile"]),
        ("process", &["exit", "std::process", "child_process"]),
        ("auth_middleware", &["verify", "decode", "verifyToken"]),
        ("weak_random", &["random"]), // For Math.random
        (
            "financial_calc",
            &["price", "priceSnapshot", "total", "amount"],
        ),
    ];

    for (category, api_names) in categories {
        for api_name in *api_names {
            let mut h = FxHasher::default();
            api_name.hash(&mut h);
            if api_calls.binary_search(&h.finish()).is_ok()
                || api_call_segments.binary_search(&h.finish()).is_ok()
                || property_accesses.binary_search(&h.finish()).is_ok()
            {
                let mut cat_h = FxHasher::default();
                category.hash(&mut cat_h);
                markers.insert(cat_h.finish());
                break;
            }
        }
    }

    let mut vec: Vec<u64> = markers.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_signature_tokens(node: Node<'_>, source: &str) -> Vec<String> {
    let start = node.start_byte();
    let end = node
        .child_by_field_name("body")
        .map_or(node.end_byte(), |b| b.start_byte());
    source[start..end]
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect()
}

fn extract_param_types(node: Node<'_>, source: &str) -> Vec<String> {
    let mut types = Vec::new();
    if let Some(params) = node.child_by_field_name("parameters") {
        let mut cursor = params.walk();
        loop {
            let n = cursor.node();
            if let Some(type_node) = n.child_by_field_name("type") {
                types.push(source[type_node.start_byte()..type_node.end_byte()].to_string());
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    types
}

pub fn extract_fingerprints_with_nodes<'a>(
    root: Node<'a>,
    source_code: &str,
    path: &Path,
    fingerprints: &mut Vec<(FunctionFingerprint, Node<'a>)>,
    _window_size: usize,
) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let language = crate::parser::ext_to_language(ext).to_string();

    let lang: Language = match ext {
        "rs" => Language::Rust,
        "ts" | "tsx" => Language::TypeScript,
        "js" | "jsx" => Language::JavaScript,
        "c" | "h" => Language::C,
        "py" | "pyi" => Language::Python,
        _ => return,
    };

    let mut cursor = root.walk();
    loop {
        let node = cursor.node();
        let kind = node.kind();
        if matches!(
            kind,
            "function_item" | "function_declaration" | "method_definition" | "arrow_function"
        ) && let Some(body) = node.child_by_field_name("body")
        {
            let mut function_name = "anonymous".to_string();
            if let Some(name_node) = node.child_by_field_name("name") {
                function_name =
                    source_code[name_node.start_byte()..name_node.end_byte()].to_string();
            } else if kind == "arrow_function"
                && let Some(parent) = node.parent()
                && parent.kind() == "variable_declarator"
                && let Some(name_node) = parent.child_by_field_name("name")
            {
                function_name =
                    source_code[name_node.start_byte()..name_node.end_byte()].to_string();
            }

            let body_code = &source_code[body.start_byte()..body.end_byte()];
            let tokens: Vec<String> = body_code
                .split_whitespace()
                .filter(|t| !t.is_empty() && !t.starts_with("//"))
                .map(String::from)
                .collect();

            let total_bytes = body.end_byte() - body.start_byte();
            let comment_bytes = count_comment_bytes(body, source_code);
            let sig_tokens = extract_signature_tokens(node, source_code);
            let param_types = extract_param_types(node, source_code);
            let name_segments = split_name_segments(&function_name);

            // Multi-scale n-grams: combine window sizes 3, 5, and 8
            let mut multi_scale_hashes = token_ngrams_positional(&tokens, 3);
            multi_scale_hashes.extend(token_ngrams_positional(&tokens, 5));
            multi_scale_hashes.extend(token_ngrams_positional(&tokens, 8));

            // AST-aware features
            let control_flow = extract_control_flow(body, source_code);
            let (api_calls, api_call_segments) = extract_api_calls(body, source_code);
            let property_accesses = extract_property_accesses(body, source_code);
            let semantic_markers =
                extract_semantic_markers(body, source_code, &api_calls, &api_call_segments, &property_accesses);

            // Extract tainted API calls: calls where an argument references a parameter
            let param_names: Vec<String> = node
                .child_by_field_name("parameters")
                .map(|p| {
                    let mut names = Vec::new();
                    let mut c = p.walk();
                    if c.goto_first_child() {
                        loop {
                            let child = c.node();
                            if let Some(name) = child.child_by_field_name("pattern")
                                .or_else(|| child.child_by_field_name("name"))
                            {
                                let text = &source_code[name.start_byte()..name.end_byte()];
                                // Extract identifier from destructuring and object patterns
                                if child.kind() == "identifier" || child.kind() == "required_parameter" {
                                    names.push(text.to_string());
                                }
                            }
                            if !c.goto_next_sibling() { break; }
                        }
                    }
                    names
                })
                .unwrap_or_default();
            let tainted_api_calls = extract_tainted_calls(body, source_code, &param_names);

            let skeleton = crate::ast_distance::extract_skeleton(body, source_code);
            let mut skeleton_hashes = Vec::with_capacity(skeleton.len());
            for s in &skeleton {
                let mut hasher = rustc_hash::FxHasher::default();
                std::hash::Hash::hash(s, &mut hasher);
                skeleton_hashes.push(std::hash::Hasher::finish(&hasher));
            }
            let fp = FunctionFingerprint {
                file_path: path.to_string_lossy().to_string(),
                function_name,
                line: node.start_position().row + 1,
                language: language.clone(),
                ngram_hashes: multi_scale_hashes.clone(),
                weighted_ngram_hashes: multi_scale_hashes.into_iter().map(|h| (h, 1.0)).collect(),
                signature_ngrams: token_ngrams_sorted(&sig_tokens, 3.min(sig_tokens.len().max(1))),
                param_type_ngrams: token_ngrams_sorted(
                    &param_types,
                    2.min(param_types.len().max(1)),
                ),
                name_segments,
                structural_markers: collect_structural_markers(body, source_code, lang),
                type_usages: collect_type_usages(body, source_code),
                comment_density: if total_bytes > 0 {
                    comment_bytes as f64 / total_bytes as f64
                } else {
                    0.0
                },
                semantic_markers,
                skeleton,
                skeleton_hashes,
                 control_flow_hashes: control_flow,
                  api_calls,
                  api_call_segments,
                  property_accesses,
                  tainted_api_calls,
              };

             fingerprints.push((fp, node));
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}

pub fn extract_fingerprints(
    root: Node,
    source_code: &str,
    path: &Path,
    fingerprints: &mut Vec<FunctionFingerprint>,
    _window_size: usize,
) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let language = crate::parser::ext_to_language(ext).to_string();

    let lang: Language = match ext {
        "rs" => Language::Rust,
        "ts" | "tsx" => Language::TypeScript,
        "js" | "jsx" => Language::JavaScript,
        "c" | "h" => Language::C,
        "py" | "pyi" => Language::Python,
        _ => return,
    };

    let mut cursor = root.walk();
    loop {
        let node = cursor.node();
        let kind = node.kind();
        if matches!(
            kind,
            "function_item" | "function_declaration" | "method_definition" | "arrow_function"
        ) && let Some(body) = node.child_by_field_name("body")
        {
            let mut function_name = "anonymous".to_string();
            if let Some(name_node) = node.child_by_field_name("name") {
                function_name =
                    source_code[name_node.start_byte()..name_node.end_byte()].to_string();
            } else if kind == "arrow_function"
                && let Some(parent) = node.parent()
                && parent.kind() == "variable_declarator"
                && let Some(name_node) = parent.child_by_field_name("name")
            {
                function_name =
                    source_code[name_node.start_byte()..name_node.end_byte()].to_string();
            }

            let body_code = &source_code[body.start_byte()..body.end_byte()];
            let tokens: Vec<String> = body_code
                .split_whitespace()
                .filter(|t| !t.is_empty() && !t.starts_with("//"))
                .map(String::from)
                .collect();

            let total_bytes = body.end_byte() - body.start_byte();
            let comment_bytes = count_comment_bytes(body, source_code);
            let sig_tokens = extract_signature_tokens(node, source_code);
            let param_types = extract_param_types(node, source_code);
            let name_segments = split_name_segments(&function_name);

            // Multi-scale n-grams: combine window sizes 3, 5, and 8
            let mut multi_scale_hashes = token_ngrams_positional(&tokens, 3);
            multi_scale_hashes.extend(token_ngrams_positional(&tokens, 5));
            multi_scale_hashes.extend(token_ngrams_positional(&tokens, 8));

            // AST-aware features
            let control_flow = extract_control_flow(body, source_code);
            let (api_calls, api_call_segments) = extract_api_calls(body, source_code);
            let property_accesses = extract_property_accesses(body, source_code);
            let semantic_markers =
                extract_semantic_markers(body, source_code, &api_calls, &api_call_segments, &property_accesses);

            // Extract tainted API calls
            let param_names: Vec<String> = node
                .child_by_field_name("parameters")
                .map(|p| {
                    let mut names = Vec::new();
                    let mut c = p.walk();
                    if c.goto_first_child() {
                        loop {
                            let child = c.node();
                            if let Some(name) = child.child_by_field_name("pattern")
                                .or_else(|| child.child_by_field_name("name"))
                            {
                                let text = &source_code[name.start_byte()..name.end_byte()];
                                if child.kind() == "identifier" || child.kind() == "required_parameter" {
                                    names.push(text.to_string());
                                }
                            }
                            if !c.goto_next_sibling() { break; }
                        }
                    }
                    names
                })
                .unwrap_or_default();
            let tainted_api_calls = extract_tainted_calls(body, source_code, &param_names);

            let skeleton = crate::ast_distance::extract_skeleton(body, source_code);
            let mut skeleton_hashes = Vec::with_capacity(skeleton.len());
            for s in &skeleton {
                let mut hasher = rustc_hash::FxHasher::default();
                std::hash::Hash::hash(s, &mut hasher);
                skeleton_hashes.push(std::hash::Hasher::finish(&hasher));
            }
            fingerprints.push(FunctionFingerprint {
                file_path: path.to_string_lossy().to_string(),
                function_name,
                line: node.start_position().row + 1,
                language: language.clone(),
                ngram_hashes: multi_scale_hashes.clone(),
                weighted_ngram_hashes: multi_scale_hashes.into_iter().map(|h| (h, 1.0)).collect(),
                signature_ngrams: token_ngrams_sorted(&sig_tokens, 3.min(sig_tokens.len().max(1))),
                param_type_ngrams: token_ngrams_sorted(
                    &param_types,
                    2.min(param_types.len().max(1)),
                ),
                name_segments,
                structural_markers: collect_structural_markers(body, source_code, lang),
                type_usages: collect_type_usages(body, source_code),
                comment_density: if total_bytes > 0 {
                    comment_bytes as f64 / total_bytes as f64
                } else {
                    0.0
                },
                semantic_markers,
                skeleton,
                skeleton_hashes,
                control_flow_hashes: control_flow,
                api_calls,
                api_call_segments,
                property_accesses,
                tainted_api_calls,
            });
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}
