use rustc_hash::{FxHashSet, FxHasher};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::path::Path;
use tree_sitter::Node;

#[cfg(feature = "fingerprinting")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub language: String,
    pub ngram_hashes: FxHashSet<u64>,
    pub signature_ngrams: FxHashSet<u64>,
    pub param_type_ngrams: FxHashSet<u64>,
    pub name_segments: Vec<String>,
    pub structural_markers: FxHashSet<u64>,
    pub type_usages: Vec<String>,
    pub comment_density: f64,
}

#[cfg(not(feature = "fingerprinting"))]
pub type FunctionFingerprint = ();

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

fn collect_structural_markers(node: Node<'_>, source: &str) -> FxHashSet<u64> {
    let mut markers = FxHashSet::default();
    let mut cursor = node.walk();
    let mut hasher = FxHasher::default();
    node.kind().hash(&mut hasher);
    markers.insert(hasher.finish());
    let _ = source;
    loop {
        if cursor.goto_first_child() {
            let n = cursor.node();
            let mut h = FxHasher::default();
            n.kind().hash(&mut h);
            markers.insert(h.finish());
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                let n = cursor.node();
                let mut h = FxHasher::default();
                n.kind().hash(&mut h);
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

fn count_comment_bytes(node: Node<'_>, source: &str) -> usize {
    let mut total = 0;
    let mut cursor = node.walk();
    let _ = source;
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

#[cfg(feature = "fingerprinting")]
pub fn extract_fingerprints(
    root: Node,
    source_code: &str,
    path: &Path,
    fingerprints: &mut Vec<FunctionFingerprint>,
    window_size: usize,
) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let language = match ext {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "yml" | "yaml" => "yaml",
        _ => "unknown",
    }
    .to_string();

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

            fingerprints.push(FunctionFingerprint {
                file_path: path.to_string_lossy().to_string(),
                function_name,
                line: node.start_position().row + 1,
                language: language.clone(),
                ngram_hashes: token_ngrams(&tokens, window_size),
                signature_ngrams: token_ngrams(&sig_tokens, 3.min(sig_tokens.len().max(1))),
                param_type_ngrams: token_ngrams(&param_types, 2.min(param_types.len().max(1))),
                name_segments,
                structural_markers: collect_structural_markers(body, source_code),
                type_usages: collect_type_usages(body, source_code),
                #[allow(clippy::cast_precision_loss)]
                comment_density: if total_bytes > 0 {
                    comment_bytes as f64 / total_bytes as f64
                } else {
                    0.0
                },
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
