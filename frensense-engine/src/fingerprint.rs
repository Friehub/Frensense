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
    pub ngram_hashes: FxHashSet<u64>,
    pub weighted_ngram_hashes: FxHashMap<u64, f32>,
    pub signature_ngrams: FxHashSet<u64>,
    pub param_type_ngrams: FxHashSet<u64>,
    pub name_segments: Vec<String>,
    pub structural_markers: FxHashSet<u64>,
    pub type_usages: Vec<String>,
    pub comment_density: f64,
    /// Semantic markers: hashes of semantic features like "contains exec call"
    /// These help discriminate between different vulnerability types
    pub semantic_markers: FxHashSet<u64>,
    /// Structural skeleton: list of AST node kinds (identifiers/literals removed)
    /// Used for AST edit distance computation
    pub skeleton: Vec<String>,
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

/// Apply IDF weights to a fingerprint's weighted_ngram_hashes.
pub fn apply_idf_weights(
    fingerprint: &mut FunctionFingerprint,
    idf_weights: &FxHashMap<u64, f32>,
) {
    for (hash, weight) in &mut fingerprint.weighted_ngram_hashes {
        if let Some(&idf) = idf_weights.get(hash) {
            *weight = idf;
        }
    }
}

/// M9: Position-weighted n-gram hashing.
/// Combines position with token hash so that `return` at line 5 differs from `return` at line 50.
fn token_ngrams_positional(tokens: &[String], window_size: usize) -> FxHashSet<u64> {
    if tokens.len() < window_size {
        return FxHashSet::default();
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
    hashes
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

fn collect_structural_markers(node: Node<'_>, _source: &str, language: Language) -> FxHashSet<u64> {
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
                return markers;
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

/// Extract semantic markers from function body.
/// These markers identify what APIs/patterns the function uses,
/// helping discriminate between different vulnerability types.
fn extract_semantic_markers(node: Node<'_>, source: &str) -> FxHashSet<u64> {
    let mut markers = FxHashSet::default();
    let body_text = &source[node.start_byte()..node.end_byte()];

    // Semantic categories and their keywords
    let categories = [
        // Database operations
        ("db_query", &["query", "execute", "raw_query", "format_sql", "sql_query"] as &[&str]),
        ("db_write", &["insert", "update", "upsert", "create"] as &[&str]),
        // Command execution
        ("cmd_exec", &["exec", "system", "spawn", "popen", "shell"] as &[&str]),
        // Code evaluation
        ("code_eval", &["eval", "Function(", "new Function"] as &[&str]),
        // File operations
        ("file_read", &["readFile", "readFileSync", "createReadStream", "read_to_string"] as &[&str]),
        ("file_write", &["writeFile", "writeFileSync", "createWriteStream", "write"] as &[&str]),
        // DOM manipulation
        ("dom_xss", &["innerHTML", "outerHTML", "document.write", "insertAdjacentHTML"] as &[&str]),
        // HTTP operations
        ("http_request", &["fetch", "axios", "request", "http.get", "http.post"] as &[&str]),
        // URL operations
        ("url_redirect", &["location.href", "location.assign", "redirect", "res.redirect"] as &[&str]),
        // Crypto operations
        ("crypto_weak", &["md5", "sha1", "createHash"] as &[&str]),
        ("crypto_strong", &["sha256", "sha512", "bcrypt", "argon2"] as &[&str]),
        // Serialization
        ("deserialize", &["JSON.parse", "from_str", "loads", "deserialize"] as &[&str]),
        // Desensitization
        ("sanitize", &["sanitize", "escape", "encode", "validate"] as &[&str]),
        // Regex
        ("regex", &["Regex::new", "new RegExp", "re.compile"] as &[&str]),
        // Process
        ("process", &["process.exit", "std::process", "child_process"] as &[&str]),
    ];

    for (category, keywords) in &categories {
        for keyword in *keywords {
            if body_text.contains(keyword) {
                let mut hasher = FxHasher::default();
                category.hash(&mut hasher);
                markers.insert(hasher.finish());
                break; // Only need one match per category
            }
        }
    }

    markers
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

pub fn extract_fingerprints(
    root: Node,
    source_code: &str,
    path: &Path,
    fingerprints: &mut Vec<FunctionFingerprint>,
    window_size: usize,
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

            let positional_hashes = token_ngrams_positional(&tokens, window_size);
            let semantic_markers = extract_semantic_markers(body, source_code);
            let skeleton = crate::ast_distance::extract_skeleton(body, source_code);

            fingerprints.push(FunctionFingerprint {
                file_path: path.to_string_lossy().to_string(),
                function_name,
                line: node.start_position().row + 1,
                language: language.clone(),
                ngram_hashes: positional_hashes.clone(),
                weighted_ngram_hashes: positional_hashes.into_iter().map(|h| (h, 1.0)).collect(),
                signature_ngrams: token_ngrams(&sig_tokens, 3.min(sig_tokens.len().max(1))),
                param_type_ngrams: token_ngrams(&param_types, 2.min(param_types.len().max(1))),
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
