// SPDX-License-Identifier: MIT

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::{Hash, Hasher};
use std::path::Path;
use tree_sitter::Node;

use crate::lang::kinds::AbstractKind;
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

    /// Hash of the ordered control-flow event sequence (branch, return, call, ...).
    /// Separate from control_flow_hashes (which uses Jaccard on a set) so the
    /// scorer can apply an exact-match penalty when ordering differs.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub control_flow_sequence_hash: u64,
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

    /// Motif hashes: hashes of the canonical motif name for each API call
    /// that belongs to a known motif group (e.g. exec/spawn/Command::new →
    /// all hash as "CommandExecutionSink"). These replace literal call hashes
    /// for cross-variant matching without requiring separate corpus patterns.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub motif_hashes: Vec<u64>,

    /// Data-flow path hashes: hashes of abstract source→sink chains found
    /// inside this function. Each hash represents a path like:
    ///   "UserInputSource → assignment → call → CommandExecutionSink"
    /// Variable names and helper names are replaced by their motif category,
    /// making the path robust to renaming.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub data_flow_path_hashes: Vec<u64>,

    /// Raw call target name strings (e.g. "exec", "child_process.exec", "Command::new").
    /// Populated during fingerprint extraction for use in evidence reporting
    /// AND in function_role.rs for domain-specific role classification.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub raw_call_names: Vec<String>,

    /// Parameter names extracted from the function signature.
    /// Used by the role classifier to distinguish HttpHandler (has `req`/`request`)
    /// from utility helpers that merely call response methods.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub param_names: Vec<String>,

    /// Hashes of API calls where at least one argument is (or contains) a function parameter.
    /// E.g., `exec(cmd)` where `cmd` is a param → hash of `"exec"` is included.
    /// `exec("ls")` where `"ls"` is a constant → NOT included.
    /// Separated from `api_calls` so scoring can distinguish tainted from untainted sinks.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub tainted_api_calls: Vec<u64>,

    #[cfg_attr(feature = "serialize", serde(default))]
    pub config_literal_hashes: Vec<u64>,

    /// Argument call types: hashes of (function_segment, arg_position, arg_ast_kind)
    /// for each call expression argument. Captures whether calls receive
    /// string literals, template strings, binary expressions, objects, etc.
    /// Enables distinguishing `query(concat_string, object)` from
    /// `query(literal_string, object_with_replacements)`.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub argument_call_types: Vec<u64>,

    /// String literal patterns: hashes of content patterns found in string/template
    /// literal arguments to calls. Includes markers for:
    ///   - SQL keywords in strings (SELECT, FROM, WHERE, etc.)
    ///   - Binary expression concatenation (`"SELECT " + userId`)
    ///   - Template interpolation (`\`SELECT * FROM ${id}\``)
    ///   - Placeholder patterns (`:param` in parameterized queries)
    ///     Enables distinguishing vulnerable SQL concatenation from safe parameterized queries.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub literal_pattern_hashes: Vec<u64>,

    /// Whether this function/method has a routing decorator (e.g. `@Get`, `@Post`).
    /// Populated during fingerprint extraction for NestJS/routing-controllers/tsoa detection.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub has_http_decorator: bool,

    /// Whether this function is referenced as a handler in a route registration
    /// (e.g. `app.get('/path', fn)`) or is an inline arrow function passed directly
    /// to a router method. Signals the function is an HttpHandler.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub is_registered_handler: bool,

    /// Export handler kind detected from file-level export patterns
    /// (Next.js App Router, SvelteKit, Cloudflare Workers, AWS Lambda, etc.).
    /// None means either not an export or not a recognized framework export.
    #[cfg_attr(feature = "serialize", serde(default))]
    pub export_handler_kind: Option<crate::export_matcher::ExportHandlerKind>,
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

/// Normalize a token to its canonical form so that structurally equivalent
/// constructs (for/while, if/switch/match, helper/inline) hash identically.
/// This is applied BEFORE n-gram computation to make fingerprints invariant
/// to common code transformations that attackers use to evade detection.
fn normalize_token(tok: &str) -> &str {
    match tok {
        // Loop family → canonical "loop"
        "while" | "for" | "loop" | "do" => "loop",
        // Branch family → canonical "branch"
        "if" | "switch" | "match" => "branch",
        // Error handling family → canonical "catch"
        "catch" | "except" | "rescue" | "recover" => "catch",
        // Async/await family
        "async" | "await" | "yield" | "suspend" => "async_op",
        // Return/break family
        "return" | "break" | "continue" | "throw" | "raise" => "exit",
        // Everything else unchanged
        other => other,
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

    let kind = abstract_kind(node.kind(), language);
    if kind != AbstractKind::Other {
        let mut hasher = FxHasher::default();
        kind.hash(&mut hasher);
        markers.insert(hasher.finish());
    }

    loop {
        if cursor.goto_first_child() {
            let n = cursor.node();
            let kind = abstract_kind(n.kind(), language);
            if kind != AbstractKind::Other {
                let mut h = FxHasher::default();
                kind.hash(&mut h);
                markers.insert(h.finish());
            }
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                let n = cursor.node();
                let kind = abstract_kind(n.kind(), language);
                if kind != AbstractKind::Other {
                    let mut h = FxHasher::default();
                    kind.hash(&mut h);
                    markers.insert(h.finish());
                }
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

/// Extract the ordered control-flow sequence as a single hash.
/// Used by the scorer to penalize order mismatches (e.g. check→delete vs
/// delete→check) that Jaccard on the full control_flow_hashes set dilutes.
fn extract_cf_sequence_hash(node: Node<'_>, source: &str) -> u64 {
    let ordered = collect_cf_sequence(node, source);
    if ordered.is_empty() {
        return 0;
    }
    let mut seq_h = FxHasher::default();
    "cf_sequence".hash(&mut seq_h);
    ordered.hash(&mut seq_h);
    seq_h.finish()
}

/// Collect control-flow event names in document order (pre-order traversal).
fn collect_cf_sequence(node: Node<'_>, source: &str) -> Vec<String> {
    let mut events = Vec::new();
    collect_cf_seq_recursive(node, source, &mut events);
    events
}

fn collect_cf_seq_recursive(node: Node<'_>, _source: &str, events: &mut Vec<String>) {
    let kind = node.kind();
    let event = match kind {
        "if_expression"
        | "if_statement"
        | "match_expression"
        | "match_statement"
        | "switch_statement"
        | "switch_expression"
        | "conditional_expression" => Some("branch"),
        "loop_expression" | "while_expression" | "for_expression" => Some("loop"),
        "return_expression" | "return_statement" => Some("return"),
        "break_expression" => Some("break"),
        "try_expression" | "try_statement" => Some("try"),
        "catch_clause" | "catch_block" => Some("catch"),
        _ => None,
    };
    if let Some(e) = event {
        events.push(e.to_string());
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_cf_seq_recursive(cursor.node(), _source, events);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn extract_cf_recursive(
    node: Node<'_>,
    _source: &str,
    path: &mut Vec<String>,
    hashes: &mut FxHashSet<u64>,
) {
    let kind = node.kind();

    // Record control flow nodes (normalized to canonical forms)
    match kind {
        "if_expression"
        | "if_statement"
        | "match_expression"
        | "match_statement"
        | "switch_statement"
        | "switch_expression"
        | "conditional_expression" => {
            path.push("branch".to_string());
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
            extract_cf_recursive(child, _source, path, hashes);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    // Pop path when leaving control flow nodes
    match kind {
        "if_expression"
        | "if_statement"
        | "match_expression"
        | "match_statement"
        | "switch_statement"
        | "switch_expression"
        | "conditional_expression"
        | "loop_expression"
        | "while_expression"
        | "for_expression"
        | "return_expression"
        | "return_statement"
        | "break_expression"
        | "try_expression"
        | "try_statement"
        | "catch_clause"
        | "catch_block" => {
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

/// Extract argument call types: hashes of (function_segment, arg_position, arg_kind)
/// for each call expression. Captures whether calls receive string literals,
/// binary expressions (concat), template strings, objects, identifiers, etc.
/// Enables distinguishing `query(concat_string, obj)` from `query(literal, obj)`.
fn extract_argument_call_types(node: Node<'_>, source: &str) -> Vec<u64> {
    let mut arg_types = rustc_hash::FxHashSet::default();
    extract_arg_types_recursive(node, source, &mut arg_types);
    let mut vec: Vec<u64> = arg_types.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_arg_types_recursive(
    node: Node<'_>,
    source: &str,
    arg_types: &mut rustc_hash::FxHashSet<u64>,
) {
    if node.kind() == "call_expression" {
        let func_segment = node
            .child_by_field_name("function")
            .map(|f| {
                let name = &source[f.start_byte()..f.end_byte()];
                name.rsplit(['.', ':']).next().unwrap_or(name).to_string()
            })
            .unwrap_or_default();

        if let Some(args_node) = node.child_by_field_name("arguments") {
            let mut pos = 0usize;
            let mut cursor = args_node.walk();
            if cursor.goto_first_child() {
                loop {
                    let arg = cursor.node();
                    let kind = arg.kind();
                    // Skip punctuation/delimiters (commas, parens)
                    if kind != "," && kind != "(" && kind != ")" && kind != ";" {
                        let mut h = rustc_hash::FxHasher::default();
                        func_segment.hash(&mut h);
                        pos.hash(&mut h);
                        kind.hash(&mut h);
                        arg_types.insert(h.finish());

                        // If the arg itself is a call expression, also hash the result type
                        if kind == "call_expression" {
                            let mut h2 = rustc_hash::FxHasher::default();
                            func_segment.hash(&mut h2);
                            pos.hash(&mut h2);
                            "call_result".hash(&mut h2);
                            arg_types.insert(h2.finish());
                        }

                        pos += 1;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_arg_types_recursive(cursor.node(), source, arg_types);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract string literal patterns from call arguments.
/// Detects:
///   - binary_expression (string concatenation: "SELECT " + userId)
///   - template_string with interpolation (`SELECT * FROM ${id}`)
///   - string literal containing SQL keywords (SELECT, FROM, WHERE, etc.)
///   - string literal containing parameterized placeholders (:param, ?)
///     Hashes as (function_segment, pattern_type) for each call.
fn extract_literal_patterns(node: Node<'_>, source: &str) -> Vec<u64> {
    let mut patterns = rustc_hash::FxHashSet::default();
    extract_literal_patterns_recursive(node, source, &mut patterns);
    let mut vec: Vec<u64> = patterns.into_iter().collect();
    vec.sort_unstable();
    vec
}

fn extract_literal_patterns_recursive(
    node: Node<'_>,
    source: &str,
    patterns: &mut rustc_hash::FxHashSet<u64>,
) {
    if node.kind() == "call_expression" {
        let func_segment = node
            .child_by_field_name("function")
            .map(|f| {
                let name = &source[f.start_byte()..f.end_byte()];
                name.rsplit(['.', ':']).next().unwrap_or(name).to_string()
            })
            .unwrap_or_default();

        if let Some(args_node) = node.child_by_field_name("arguments") {
            let mut pos = 0usize;
            let mut cursor = args_node.walk();
            if cursor.goto_first_child() {
                loop {
                    let arg = cursor.node();
                    let kind = arg.kind();
                    if kind != "," && kind != "(" && kind != ")" && kind != ";" {
                        let arg_text = &source[arg.start_byte()..arg.end_byte()];

                        let pattern_type = match kind {
                            "binary_expression" if arg_text.contains('+') => {
                                let arg_upper = arg_text.to_uppercase();
                                if arg_upper.contains("SELECT")
                                    || arg_upper.contains("FROM")
                                    || arg_upper.contains("WHERE")
                                    || arg_upper.contains("INSERT")
                                    || arg_upper.contains("UPDATE")
                                    || arg_upper.contains("DELETE")
                                {
                                    "sql_concat"
                                } else {
                                    "string_concat"
                                }
                            }
                            "template_string" => {
                                let interp = arg_text.contains("${");
                                let arg_upper = arg_text.to_uppercase();
                                let has_sql = arg_upper.contains("SELECT")
                                    || arg_upper.contains("FROM")
                                    || arg_upper.contains("WHERE");
                                if interp && has_sql {
                                    "sql_template_interp"
                                } else if interp {
                                    "template_interp"
                                } else if has_sql {
                                    "sql_template_literal"
                                } else {
                                    "template_literal"
                                }
                            }
                            _ => {
                                // For string literals and other types, check content
                                let upper = arg_text.to_uppercase();
                                let has_sql = upper.contains("SELECT")
                                    || upper.contains("FROM")
                                    || upper.contains("WHERE")
                                    || upper.contains("INSERT")
                                    || upper.contains("DELETE");
                                let has_params = arg_text.contains(':')
                                    && (arg_text.contains(":param")
                                        || arg_text.contains(":value")
                                        || arg_text.contains(":id"));
                                let has_qmark = arg_text.contains('?');
                                let is_parametrized = has_params || has_qmark;

                                if is_parametrized && has_sql {
                                    "sql_parametrized_literal"
                                } else if has_sql {
                                    "sql_literal"
                                } else if is_parametrized {
                                    "parametrized_literal"
                                } else {
                                    // Not a notable pattern — skip
                                    pos += 1;
                                    if !cursor.goto_next_sibling() {
                                        break;
                                    }
                                    continue;
                                }
                            }
                        };

                        let mut h = rustc_hash::FxHasher::default();
                        func_segment.hash(&mut h);
                        pattern_type.hash(&mut h);
                        pos.hash(&mut h);
                        patterns.insert(h.finish());

                        pos += 1;
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_literal_patterns_recursive(cursor.node(), source, patterns);
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
fn has_param_ref(node: Node<'_>, _source: &str, _param_names: &[String]) -> bool {
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
                    if has_param_ref(cursor.node(), _source, _param_names) {
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
/// Collect raw call target name strings from a function body (for motif lookup).
fn collect_raw_call_names(node: Node<'_>, source: &str) -> Vec<String> {
    let mut names = Vec::new();
    collect_raw_calls_recursive(node, source, &mut names);
    names
}

fn collect_raw_calls_recursive(node: Node<'_>, source: &str, names: &mut Vec<String>) {
    if node.kind() == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            names.push(source[func.start_byte()..func.end_byte()].to_string());
        }
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            collect_raw_calls_recursive(cursor.node(), source, names);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Given raw call target name strings, produce sorted deduplicated motif hashes.
fn extract_motif_hashes(
    call_names: &[String],
    motif_lookup: &rustc_hash::FxHashMap<String, &'static str>,
) -> Vec<u64> {
    use std::hash::{Hash, Hasher};
    let mut set = rustc_hash::FxHashSet::default();
    for call in call_names {
        // Check the full call name
        if let Some(&motif_name) = motif_lookup.get(call.as_str()) {
            let mut h = rustc_hash::FxHasher::default();
            motif_name.hash(&mut h);
            set.insert(h.finish());
        }
        // Also check the last segment (e.g. "spawn" from "child_process.spawn")
        if let Some(pos) = call.rfind("::").or_else(|| call.rfind('.')) {
            let seg = &call[pos + 1..];
            if let Some(&motif_name) = motif_lookup.get(seg) {
                let mut h = rustc_hash::FxHasher::default();
                motif_name.hash(&mut h);
                set.insert(h.finish());
            }
        }
    }
    let mut vec: Vec<u64> = set.into_iter().collect();
    vec.sort_unstable();
    vec
}

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
            } else if let Some(inferred) =
                crate::route_registry::infer_function_name(node, source_code)
            {
                function_name = inferred;
            }

            let body_code = &source_code[body.start_byte()..body.end_byte()];
            let tokens: Vec<String> = body_code
                .split_whitespace()
                .filter(|t| !t.is_empty() && !t.starts_with("//"))
                .map(|t| normalize_token(t).to_string())
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
            let cf_sequence_hash = extract_cf_sequence_hash(body, source_code);
            let (api_calls, api_call_segments) = extract_api_calls(body, source_code);
            let property_accesses = extract_property_accesses(body, source_code);
            let semantic_markers = extract_semantic_markers(
                body,
                source_code,
                &api_calls,
                &api_call_segments,
                &property_accesses,
            );

            // Extract tainted API calls: calls where an argument references a parameter
            let param_names: Vec<String> = node
                .child_by_field_name("parameters")
                .map(|p| {
                    let mut names = Vec::new();
                    let mut c = p.walk();
                    if c.goto_first_child() {
                        loop {
                            let child = c.node();
                            if let Some(name) = child
                                .child_by_field_name("pattern")
                                .or_else(|| child.child_by_field_name("name"))
                            {
                                let text = &source_code[name.start_byte()..name.end_byte()];
                                // Extract identifier from destructuring and object patterns
                                if child.kind() == "identifier"
                                    || child.kind() == "required_parameter"
                                {
                                    names.push(text.to_string());
                                }
                            }
                            if !c.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    names
                })
                .unwrap_or_default();
            let tainted_api_calls = extract_tainted_calls(body, source_code, &param_names);
            let raw_call_names = collect_raw_call_names(body, source_code);
            let motif_hashes =
                extract_motif_hashes(&raw_call_names, &crate::corpus::motifs::MOTIF_LOOKUP);
            let data_flow_path_hashes =
                crate::corpus::flow_fingerprint::extract_flow_paths(body, source_code);
            let argument_call_types = extract_argument_call_types(body, source_code);
            let literal_pattern_hashes = extract_literal_patterns(body, source_code);

            let skeleton = crate::ast_distance::extract_skeleton(body, source_code);
            let mut skeleton_hashes = Vec::with_capacity(skeleton.len());
            for s in &skeleton {
                let mut hasher = rustc_hash::FxHasher::default();
                std::hash::Hash::hash(s, &mut hasher);
                skeleton_hashes.push(std::hash::Hasher::finish(&hasher));
            }
            let has_http_decorator =
                crate::decorator::has_routing_decorator(node, source_code).is_some();
            let is_registered_handler =
                crate::route_registry::is_function_registered_in_file(node, source_code, "")
                    || crate::route_registry::is_inline_registered_handler(node, source_code);

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
                type_usages: {
                    let mut tu = collect_type_usages(body, source_code);
                    tu.extend(crate::decorator::collect_param_decorator_types(
                        node,
                        source_code,
                    ));
                    tu
                },
                comment_density: if total_bytes > 0 {
                    comment_bytes as f64 / total_bytes as f64
                } else {
                    0.0
                },
                semantic_markers,
                skeleton,
                skeleton_hashes,
                control_flow_hashes: control_flow,
                control_flow_sequence_hash: cf_sequence_hash,
                api_calls,
                api_call_segments,
                property_accesses,
                motif_hashes,
                data_flow_path_hashes,
                raw_call_names,
                param_names,
                tainted_api_calls,
                config_literal_hashes: Vec::new(),
                argument_call_types,
                literal_pattern_hashes,
                has_http_decorator,
                is_registered_handler,
                export_handler_kind: crate::export_matcher::classify_exported_handler(
                    node,
                    source_code,
                    &path.to_string_lossy(),
                ),
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
    window_size: usize,
) {
    let mut with_nodes = Vec::new();
    extract_fingerprints_with_nodes(root, source_code, path, &mut with_nodes, window_size);
    fingerprints.extend(with_nodes.into_iter().map(|(fp, _)| fp));
}
