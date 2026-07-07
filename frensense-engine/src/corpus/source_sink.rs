// SPDX-License-Identifier: MIT

//! Corpus-driven source/sink registry.
//!
//! Extracts source types and sink function names from positive corpus files
//! at load time. Replaces hardcoded framework type arrays and sink lists.

use rustc_hash::FxHashMap;
use std::path::Path;
use tree_sitter::Node;

/// Minimum number of distinct patterns a type/sink must appear in
/// to be promoted to the registry.
const MIN_OCCURRENCES: usize = 2;

/// Registry of source types and sink names learned from the corpus.
#[derive(Debug, Clone, Default)]
pub struct CorpusSourceSinkRegistry {
    pub source_types: FxHashMap<String, usize>,
    pub sink_names: FxHashMap<String, usize>,
}

impl CorpusSourceSinkRegistry {
    /// Check if a type annotation string is a known source type.
    pub fn is_source_type(&self, type_str: &str) -> bool {
        let clean = type_str.trim();
        self.source_types.contains_key(clean)
    }

    /// Check if a function name is a known sink.
    pub fn is_sink(&self, fn_name: &str) -> bool {
        self.sink_names.contains_key(fn_name)
    }

    /// Get source type count (for diagnostics).
    pub fn source_type_count(&self) -> usize {
        self.source_types.len()
    }

    /// Get sink name count (for diagnostics).
    pub fn sink_name_count(&self) -> usize {
        self.sink_names.len()
    }

    /// Merge another registry into this one (accumulates counts).
    pub fn merge(&mut self, other: &CorpusSourceSinkRegistry) {
        for (k, v) in &other.source_types {
            *self.source_types.entry(k.clone()).or_insert(0) += v;
        }
        for (k, v) in &other.sink_names {
            *self.sink_names.entry(k.clone()).or_insert(0) += v;
        }
    }

    /// Prune entries below `MIN_OCCURRENCES` threshold.
    pub fn prune(&mut self) {
        self.source_types
            .retain(|_, count| *count >= MIN_OCCURRENCES);
        self.sink_names.retain(|_, count| *count >= MIN_OCCURRENCES);
    }
}

/// Build a registry from a set of positive source files.
///
/// Walks each file's AST to extract:
/// - Parameter type annotations → source types
/// - Call expression callee names → sink names
pub fn build_registry(positive_files: &[String]) -> CorpusSourceSinkRegistry {
    let mut registry = CorpusSourceSinkRegistry::default();

    for source in positive_files {
        let file_sources = extract_sources_from_source(source);
        let file_sinks = extract_sinks_from_source(source);

        // Count each type/sink once per file (not once per function)
        // to avoid over-counting multi-function positive files
        let mut seen_types = std::collections::HashSet::new();
        for ty in &file_sources {
            if seen_types.insert(ty.clone()) {
                *registry.source_types.entry(ty.clone()).or_insert(0) += 1;
            }
        }

        let mut seen_sinks = std::collections::HashSet::new();
        for sink in &file_sinks {
            if seen_sinks.insert(sink.clone()) {
                *registry.sink_names.entry(sink.clone()).or_insert(0) += 1;
            }
        }
    }

    registry.prune();
    registry
}

/// Extract parameter name and type from a function parameter node.
///
/// Returns `(param_name, type_string)`. The type string includes the full
/// type annotation text (e.g., `"Request"`, `"Json<User>"`, `"String"`).
pub fn extract_param_info(param: tree_sitter::Node, source: &str) -> (String, String) {
    let mut name = String::new();
    let mut ty = String::new();

    let mut cursor = param.walk();
    for child in param.children(&mut cursor) {
        match child.kind() {
            "identifier" | "shorthand_field_identifier" | "field_identifier" if name.is_empty() => {
                name = source[child.start_byte()..child.end_byte()].to_string();
            }
            "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type"
                if ty.is_empty() =>
            {
                ty = source[child.start_byte()..child.end_byte()].to_string();
            }
            _ => {}
        }
    }

    // Fallback: regex on full text
    if name.is_empty() || ty.is_empty() {
        let text = &source[param.start_byte()..param.end_byte()];
        if let Some(caps) = regex::Regex::new(r"(\w+)\s*:\s*(.+)")
            .ok()
            .and_then(|re| re.captures(text))
        {
            if name.is_empty() {
                name = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
            }
            if ty.is_empty() {
                ty = caps
                    .get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
            }
        }
    }

    (name, ty)
}

/// Build a registry from a corpus directory by reading all positive files.
pub fn build_registry_from_dir(corpus_dir: &Path) -> CorpusSourceSinkRegistry {
    let mut positive_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(corpus_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.contains("_positive.") {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    positive_files.push(source);
                }
            }
        }
    }

    build_registry(&positive_files)
}

/// Extract source types from a source file by walking parameter type annotations.
fn extract_sources_from_source(source: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut parser = tree_sitter::Parser::new();

    let lang_name = if source.contains("fn ") {
        "rust"
    } else {
        "typescript"
    };

    let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name).ok();
    let Some(lang) = lang else { return types };
    parser.set_language(&lang).ok();
    let Some(tree) = parser.parse(source, None) else {
        return types;
    };

    extract_param_types(tree.root_node(), source, &mut types);
    types
}

/// Extract sink function names from a source file by walking call expressions.
fn extract_sinks_from_source(source: &str) -> Vec<String> {
    let mut sinks = Vec::new();
    let mut parser = tree_sitter::Parser::new();

    let lang_name = if source.contains("fn ") {
        "rust"
    } else {
        "typescript"
    };

    let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name).ok();
    let Some(lang) = lang else { return sinks };
    parser.set_language(&lang).ok();
    let Some(tree) = parser.parse(source, None) else {
        return sinks;
    };

    extract_call_names(tree.root_node(), source, &mut sinks);
    sinks
}

/// Recursively extract type annotations from function parameters.
fn extract_param_types(node: Node, source: &str, types: &mut Vec<String>) {
    let is_fn = matches!(
        node.kind(),
        "function_definition"     // TS
        | "function_declaration"  // TS
        | "arrow_function"        // TS
        | "method_definition"     // TS
        | "function_item"         // Rust
        | "function_signature_item" // Rust
    );

    if is_fn {
        if let Some(params) = node
            .child_by_field_name("parameters")
            .or_else(|| node.child_by_field_name("formal_parameters"))
        {
            let mut cursor = params.walk();
            for param in params.children(&mut cursor) {
                if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                    continue;
                }
                extract_type_from_param(param, source, types);
            }
        }
    }

    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_param_types(cursor.node(), source, types);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract the type annotation from a single parameter node.
fn extract_type_from_param(param: Node, source: &str, types: &mut Vec<String>) {
    let mut cursor = param.walk();
    for child in param.children(&mut cursor) {
        match child.kind() {
            "type_annotation" | "type_identifier" | "scoped_type_identifier" | "generic_type" => {
                let ty = source[child.start_byte()..child.end_byte()].trim();
                if !ty.is_empty() {
                    let clean = ty.trim_start_matches(':').trim();
                    types.push(clean.to_string());
                }
            }
            _ => {}
        }
    }
}

/// Recursively extract function call names from the AST.
fn extract_call_names(node: Node, source: &str, sinks: &mut Vec<String>) {
    if node.kind() == "call_expression" {
        if let Some(callee) = node
            .child_by_field_name("function")
            .or_else(|| node.child_by_field_name("callee"))
            .or_else(|| node.child(0))
        {
            let name = extract_callee_name(callee, source);
            if !name.is_empty() {
                sinks.push(name);
            }
        }
    }

    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_call_names(cursor.node(), source, sinks);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract the name from a callee node (handles method calls, paths, etc.).
fn extract_callee_name(node: Node, source: &str) -> String {
    match node.kind() {
        "identifier" | "field_identifier" => source[node.start_byte()..node.end_byte()].to_string(),
        "member_expression" => {
            // e.g., console.log → "log"
            if let Some(field) = node.child_by_field_name("field") {
                return source[field.start_byte()..field.end_byte()].to_string();
            }
            // Fallback: take last child
            if let Some(last) = node.child(node.child_count() - 1) {
                return source[last.start_byte()..last.end_byte()].to_string();
            }
            String::new()
        }
        "scoped_identifier" => {
            // e.g., std::process::Command → "Command"
            let text = source[node.start_byte()..node.end_byte()].to_string();
            text.rsplit("::").next().unwrap_or(&text).to_string()
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sources_typescript() {
        let source = "function handler(req: Request, body: Json<User>) { }";
        let types = extract_sources_from_source(source);
        assert!(
            types.contains(&"Request".to_string()),
            "should find Request type"
        );
        assert!(
            types.contains(&"Json<User>".to_string()),
            "should find Json<User> type"
        );
    }

    #[test]
    fn test_extract_sources_rust() {
        let source = "fn handler(req: Query<Params>) -> Response {\n    let x = req;\n}";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(source, None).unwrap();
        let mut types = Vec::new();
        extract_param_types(tree.root_node(), source, &mut types);
        assert!(
            !types.is_empty(),
            "should find at least one type, got: {:?}",
            types
        );
    }

    #[test]
    fn test_extract_sinks_typescript() {
        let source = "function handler() { exec(cmd); eval(code); }";
        let sinks = extract_sinks_from_source(source);
        assert!(sinks.contains(&"exec".to_string()), "should find exec sink");
        assert!(sinks.contains(&"eval".to_string()), "should find eval sink");
    }

    #[test]
    fn test_extract_sinks_member_expression() {
        let source = "function handler() { console.log(msg); document.write(html); }";
        let sinks = extract_sinks_from_source(source);
        assert!(
            sinks.contains(&"log".to_string()),
            "should find log from console.log"
        );
        assert!(
            sinks.contains(&"write".to_string()),
            "should find write from document.write"
        );
    }

    #[test]
    fn test_registry_pruning() {
        let mut registry = CorpusSourceSinkRegistry::default();
        registry.source_types.insert("Request".to_string(), 3);
        registry.source_types.insert("OneOff".to_string(), 1);
        registry.sink_names.insert("exec".to_string(), 5);
        registry.sink_names.insert("rare_sink".to_string(), 1);

        registry.prune();

        assert!(registry.is_source_type("Request"));
        assert!(!registry.is_source_type("OneOff"));
        assert!(registry.is_sink("exec"));
        assert!(!registry.is_sink("rare_sink"));
    }

    #[test]
    fn test_build_registry() {
        let files = vec![
            "function handler(req: Request) { exec(req.query); }".to_string(),
            "function process(input: Request) { exec(input.data); }".to_string(),
        ];
        let registry = build_registry(&files);
        assert!(
            registry.source_types.contains_key("Request"),
            "Request should be a source type, got: {:?}",
            registry.source_types
        );
        assert!(
            registry.sink_names.contains_key("exec"),
            "exec should be a sink, got: {:?}",
            registry.sink_names
        );
    }
}
