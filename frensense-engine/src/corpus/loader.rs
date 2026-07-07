// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::corpus::semantic::SemanticFilter;
use crate::fingerprint::{FunctionFingerprint, extract_fingerprints};

#[derive(Debug, Clone)]
pub struct CorpusPattern {
    pub id: String,
    pub positives: Vec<FunctionFingerprint>,
    pub negatives: Vec<FunctionFingerprint>,
    pub semantic_filter: Option<SemanticFilter>,
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
}

pub fn load_corpus(corpus_dir: &Path) -> Result<Vec<CorpusPattern>, String> {
    type PatternEntry = (
        Vec<FunctionFingerprint>,
        Vec<FunctionFingerprint>,
        AdvisoryText,
        Vec<FunctionFeatures>, // positive features
        Vec<FunctionFeatures>, // negative features
    );
    let mut pairs: HashMap<String, PatternEntry> = HashMap::new();

    for entry in fs::read_dir(corpus_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let is_positive = file_name.contains("_positive.");
        let is_negative = file_name.contains("_negative.");

        if !is_positive && !is_negative {
            continue;
        }

        let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let lang_name = crate::parser::ext_to_language(ext);
        if lang_name == "unknown" {
            eprintln!(
                "corpus: skipping unsupported extension '{ext}' in '{}'",
                path.display()
            );
            continue;
        }

        let mut parser = tree_sitter::Parser::new();
        let lang = crate::parser::ParserRegistry::get_language_by_name(lang_name)
            .map_err(|e| e.to_string())?;
        parser.set_language(&lang).map_err(|e| e.to_string())?;
        let Some(tree) = parser.parse(&source, None) else {
            continue;
        };

        let mut fps = Vec::new();
        extract_fingerprints(tree.root_node(), &source, &path, &mut fps, 5);

        if fps.is_empty() {
            continue;
        }

        // Collect features from all function nodes for semantic learning
        let mut all_features = Vec::new();
        collect_all_function_features(tree.root_node(), &source, &mut all_features);

        let pattern_name = extract_pattern_name(file_name);
        let entry = pairs.entry(pattern_name).or_default();
        if is_positive {
            entry.0.extend(fps);
            entry.3.extend(all_features);
            // Extract [frensense] block from positive file — primary source of advisory text
            if entry.2.observation.is_none() {
                entry.2 = parse_frensense_block(&source);
            }
        } else {
            entry.1.extend(fps);
            entry.4.extend(all_features);
        }
    }

    let mut patterns = Vec::new();
    let semantic_filters = load_semantic_filters();
    for (name, (pos, neg, comment_advisory, pos_features, neg_features)) in pairs {
        if pos.is_empty() && neg.is_empty() {
            continue;
        }
        if pos.is_empty() {
            eprintln!("Corpus warning: pattern '{name}' has negative but no positive example");
            continue;
        }
        if neg.is_empty() {
            eprintln!("Corpus warning: pattern '{name}' has positive but no negative example");
            continue;
        }

        // Priority: comment block > sidecar TOML > empty
        let toml_advisory = load_sidecar_toml(corpus_dir, &name);
        let observation = comment_advisory.observation.or(toml_advisory.observation);
        let impact = comment_advisory.impact.or(toml_advisory.impact);
        let improvement = comment_advisory.improvement.or(toml_advisory.improvement);

        // Learn semantic constraints from positive/negative examples
        let learned = if !pos_features.is_empty() && !neg_features.is_empty() {
            learn_from_features(&pos_features, &neg_features)
        } else {
            crate::corpus::semantic::LearnedConstraints::default()
        };

        // Merge: TOML manual filter takes precedence over learned constraints
        let filter = if let Some(manual) = semantic_filters.get(&name) {
            Some(manual.clone())
        } else if !learned.is_empty() {
            Some(learned.to_filter())
        } else {
            None
        };

        patterns.push(CorpusPattern {
            id: name.clone(),
            positives: pos,
            negatives: neg,
            semantic_filter: filter,
            observation,
            impact,
            improvement,
        });
    }

    Ok(patterns)
}

#[derive(Debug, Clone, Default)]
struct AdvisoryText {
    observation: Option<String>,
    impact: Option<String>,
    improvement: Option<String>,
}

/// Parse a `/// [frensense]` / `// [frensense]` / `# [frensense]` block from source.
///
/// Format:
/// ```text
/// [frensense]
/// observation: what the bug looks like
/// impact: what goes wrong
/// improvement: how to fix it
/// ```
///
/// Block ends at the first blank comment line or a non-comment line.
fn parse_frensense_block(source: &str) -> AdvisoryText {
    let mut result = AdvisoryText::default();
    let mut in_block = false;

    for line in source.lines() {
        let trimmed = line.trim();

        // Detect comment prefix
        let content = if let Some(c) = trimmed.strip_prefix("///") {
            Some(c.trim())
        } else if let Some(c) = trimmed.strip_prefix("//!") {
            Some(c.trim())
        } else if let Some(c) = trimmed.strip_prefix("//") {
            Some(c.trim())
        } else {
            trimmed.strip_prefix("#").map(str::trim)
        };

        let Some(text) = content else {
            // Non-comment line — block is over
            break;
        };

        if !in_block {
            if text == "[frensense]" {
                in_block = true;
            }
            continue;
        }

        // Empty comment line ends the block
        if text.is_empty() {
            break;
        }

        if let Some((key, value)) = text.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            if !value.is_empty() {
                match key.as_str() {
                    "observation" => result.observation = Some(value),
                    "impact" => result.impact = Some(value),
                    "improvement" => result.improvement = Some(value),
                    _ => {}
                }
            }
        }
    }

    result
}

fn load_sidecar_toml(corpus_dir: &std::path::Path, pattern_name: &str) -> AdvisoryText {
    let toml_path = corpus_dir.join(format!("{pattern_name}.toml"));
    let Ok(content) = std::fs::read_to_string(&toml_path) else {
        return AdvisoryText::default();
    };

    let Ok(doc) = content.parse::<toml::Table>() else {
        return AdvisoryText::default();
    };

    AdvisoryText {
        observation: doc
            .get("observation")
            .and_then(|v| v.as_str())
            .map(String::from),
        impact: doc.get("impact").and_then(|v| v.as_str()).map(String::from),
        improvement: doc
            .get("improvement")
            .and_then(|v| v.as_str())
            .map(String::from),
    }
}

/// Load semantic filters from the TOML file.
pub fn load_semantic_filters() -> std::collections::HashMap<String, SemanticFilter> {
    let mut filters = std::collections::HashMap::new();

    // Try to find the semantic_filters.toml file
    let possible_paths = [
        std::path::PathBuf::from("corpus/semantic_filters.toml"),
        std::path::PathBuf::from("../corpus/semantic_filters.toml"),
        std::path::PathBuf::from("../../corpus/semantic_filters.toml"),
    ];

    let content = possible_paths
        .iter()
        .find_map(|p| std::fs::read_to_string(p).ok());

    let Some(content) = content else {
        return filters;
    };

    // Simple TOML parser for our filter format
    let mut current_pattern: Option<String> = None;
    let mut current_filter = SemanticFilter::default();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Pattern header: [pattern_name]
        if line.starts_with('[') && line.ends_with(']') {
            // Save previous pattern
            if let Some(name) = current_pattern.take() {
                if !current_filter.is_empty() {
                    filters.insert(name, current_filter.clone());
                }
                current_filter = SemanticFilter::default();
            }
            current_pattern = Some(line[1..line.len() - 1].to_string());
            continue;
        }

        // Key = value pairs
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            // Parse array values: ["item1", "item2"]
            let parse_array = |s: &str| -> Vec<String> {
                s.trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|item| item.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            };

            match key {
                "contains_call_to" => {
                    current_filter.contains_call_to = parse_array(value);
                }
                "must_not_contain_call_to" => {
                    current_filter.must_not_contain_call_to = parse_array(value);
                }
                "function_name_regex" => {
                    current_filter.function_name_regex = Some(value.trim_matches('"').to_string());
                }
                "contains_node_type" => {
                    current_filter.contains_node_type = parse_array(value);
                }
                "must_not_contain_node_type" => {
                    current_filter.must_not_contain_node_type = parse_array(value);
                }
                _ => {}
            }
        }
    }

    // Save last pattern
    if let Some(name) = current_pattern {
        if !current_filter.is_empty() {
            filters.insert(name, current_filter);
        }
    }

    filters
}

fn extract_pattern_name(file_name: &str) -> String {
    let without_ext = file_name.rsplitn(2, '.').last().unwrap_or(file_name);

    without_ext
        .trim_end_matches("_positive")
        .trim_end_matches("_negative")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pattern_name() {
        assert_eq!(
            extract_pattern_name("rust_clone_in_loop_positive.rs"),
            "rust_clone_in_loop"
        );
        assert_eq!(
            extract_pattern_name("ts_command_injection_negative.ts"),
            "ts_command_injection"
        );
    }

    #[test]
    fn test_empty_directory_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_nonexistent_directory_returns_error() {
        let result = load_corpus(std::path::Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_file_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_positive.ts"), "").unwrap();
        std::fs::write(dir.path().join("test_negative.ts"), "").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty(), "Empty files should be skipped");
    }

    #[test]
    fn test_no_function_body_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        // Type declaration only — no function body
        std::fs::write(
            dir.path().join("test_positive.ts"),
            "interface Config { host: string; }",
        )
        .unwrap();
        std::fs::write(dir.path().join("test_negative.ts"), "type Foo = string;").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Files without functions should be skipped"
        );
    }

    #[test]
    fn test_bad_syntax_skipped_silently() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("bad_positive.ts"), "fn {{{ broken").unwrap();
        std::fs::write(dir.path().join("bad_negative.ts"), "fn {{{ broken").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Files with bad syntax should be skipped"
        );
    }

    #[test]
    fn test_unsupported_extension_skipped() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_positive.xyz"), "def foo(): pass").unwrap();
        std::fs::write(dir.path().join("test_negative.xyz"), "def bar(): pass").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Unsupported extensions should be skipped"
        );
    }

    #[test]
    fn test_only_positive_no_warning_no_crash() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("solo_positive.ts"),
            "function foo() { return 1; }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Positive-only should not produce a pattern"
        );
    }

    #[test]
    fn test_only_negative_no_warning_no_crash() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("solo_negative.ts"),
            "function bar() { return 2; }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(
            patterns.is_empty(),
            "Negative-only should not produce a pattern"
        );
    }

    #[test]
    fn test_non_function_files_ignored() {
        let dir = tempfile::tempdir().unwrap();
        // Files without _positive/_negative in name should be ignored
        std::fs::write(dir.path().join("readme.md"), "hello").unwrap();
        std::fs::write(dir.path().join("config.json"), "{}").unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_parse_frensense_block_rust() {
        let source = r#"/// [frensense]
/// observation: Function always returns true regardless of input.
/// impact: Malicious input passes validation unchecked.
/// improvement: Branch on input and return false for invalid values.
fn validate(input: &str) -> bool {
    true
}"#;
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("Function always returns true regardless of input.")
        );
        assert_eq!(
            advisory.impact.as_deref(),
            Some("Malicious input passes validation unchecked.")
        );
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("Branch on input and return false for invalid values.")
        );
    }

    #[test]
    fn test_parse_frensense_block_typescript() {
        let source = "// [frensense]\n// observation: sanitize returns input unchanged.\n// impact: XSS payload passes through.\n// improvement: HTML-escape entities.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("sanitize returns input unchanged.")
        );
        assert_eq!(
            advisory.impact.as_deref(),
            Some("XSS payload passes through.")
        );
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("HTML-escape entities.")
        );
    }

    #[test]
    fn test_parse_frensense_block_python() {
        let source = "# [frensense]\n# observation: No rejection on invalid token.\n# impact: Auth bypass.\n# improvement: Return None on failure.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("No rejection on invalid token.")
        );
        assert_eq!(advisory.impact.as_deref(), Some("Auth bypass."));
        assert_eq!(
            advisory.improvement.as_deref(),
            Some("Return None on failure.")
        );
    }

    #[test]
    fn test_parse_frensense_block_blank_line_ends() {
        let source = "/// [frensense]\n/// observation: Bug here.\n\n/// impact: Overwritten.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(advisory.observation.as_deref(), Some("Bug here."));
        assert_eq!(advisory.impact, None, "blank line should end the block");
    }

    #[test]
    fn test_parse_frensense_block_no_block() {
        let source = "fn foo() { return 1; }";
        let advisory = parse_frensense_block(source);
        assert!(advisory.observation.is_none());
        assert!(advisory.impact.is_none());
        assert!(advisory.improvement.is_none());
    }

    #[test]
    fn test_parse_frensense_block_partial() {
        let source = "/// [frensense]\n/// observation: Only observation provided.\n";
        let advisory = parse_frensense_block(source);
        assert_eq!(
            advisory.observation.as_deref(),
            Some("Only observation provided.")
        );
        assert!(advisory.impact.is_none());
        assert!(advisory.improvement.is_none());
    }

    #[test]
    fn test_valid_pair_loads_correctly() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("rust_foo_positive.rs"),
            "fn foo() -> i32 { 1 }",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rust_foo_negative.rs"),
            "fn foo() -> i32 { 2 }",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].id, "rust_foo");
        assert!(!patterns[0].positives.is_empty());
        assert!(!patterns[0].negatives.is_empty());
    }

    #[test]
    fn test_multi_function_positive_loads_all() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("rust_multi_positive.rs"),
            "fn a() { panic!(\"x\"); }\nfn b() { panic!(\"y\"); }\nfn c() { panic!(\"z\"); }\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("rust_multi_negative.rs"),
            "fn a() -> Result<(), String> { Ok(()) }\nfn b() -> Result<(), String> { Ok(()) }\n",
        )
        .unwrap();
        let patterns = load_corpus(dir.path()).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(
            patterns[0].positives.len(),
            3,
            "should extract all 3 functions from positive"
        );
        assert_eq!(
            patterns[0].negatives.len(),
            2,
            "should extract all 2 functions from negative"
        );
    }
}

/// Collected features from a function node for constraint learning.
#[derive(Debug, Clone, Default)]
struct FunctionFeatures {
    calls: Vec<String>,
    node_types: Vec<String>,
}

/// Collect features from a function node.
fn collect_function_features(node: tree_sitter::Node<'_>, source: &str) -> FunctionFeatures {
    let mut features = FunctionFeatures::default();

    // Collect call targets
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n.kind() == "call_expression" {
            if let Some(callee) = n
                .child_by_field_name("function")
                .or_else(|| n.child_by_field_name("callee"))
            {
                let target = source[callee.start_byte()..callee.end_byte()].to_string();
                features.calls.push(target);
            }
        }

        // Collect node types (only meaningful ones)
        let kind = n.kind();
        if !kind.is_empty() && !kind.starts_with("comment") {
            features.node_types.push(kind.to_string());
        }

        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                break;
            }
        }
        if !cursor.goto_first_child() {
            break;
        }
    }

    features.calls.sort();
    features.calls.dedup();
    features.node_types.sort();
    features.node_types.dedup();
    features
}

/// Collect features from all function nodes in an AST.
fn collect_all_function_features(
    node: tree_sitter::Node<'_>,
    source: &str,
    out: &mut Vec<FunctionFeatures>,
) {
    let kind = node.kind();
    if kind == "function_item"
        || kind == "function_declaration"
        || kind == "method_definition"
        || kind == "arrow_function"
        || kind == "function"
        || kind == "generator_function"
        || kind == "function_signature"
        || kind == "method_declaration"
    {
        out.push(collect_function_features(node, source));
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_all_function_features(child, source, out);
        }
    }
}

/// Learn semantic constraints from pre-collected features.
fn learn_from_features(
    pos_features: &[FunctionFeatures],
    neg_features: &[FunctionFeatures],
) -> crate::corpus::semantic::LearnedConstraints {
    if pos_features.is_empty() || neg_features.is_empty() {
        return crate::corpus::semantic::LearnedConstraints::default();
    }

    // Collect all call targets from positives and negatives
    let pos_calls: Vec<&str> = pos_features
        .iter()
        .flat_map(|f| f.calls.iter().map(|s| s.as_str()))
        .collect();
    let neg_calls: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.calls.iter().map(|s| s.as_str()))
        .collect();

    // Find calls in ALL positives but NOT in any negative
    let required_calls: Vec<String> = pos_features[0]
        .calls
        .iter()
        .filter(|call| {
            pos_features.iter().all(|f| f.calls.contains(*call))
                && !neg_calls.contains(&call.as_str())
        })
        .cloned()
        .collect();

    // Find calls in ALL negatives but NOT in any positive
    let forbidden_calls: Vec<String> = neg_features[0]
        .calls
        .iter()
        .filter(|call| {
            neg_features.iter().all(|f| f.calls.contains(*call))
                && !pos_calls.contains(&call.as_str())
        })
        .cloned()
        .collect();

    // Same for node types
    let pos_nts: Vec<&str> = pos_features
        .iter()
        .flat_map(|f| f.node_types.iter().map(|s| s.as_str()))
        .collect();
    let neg_nts: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.node_types.iter().map(|s| s.as_str()))
        .collect();

    // Filter out noise node types
    let noise: std::collections::HashSet<&str> = [
        "program",
        "statement_block",
        "expression_statement",
        "return_statement",
        "if_statement",
        "variable_declaration",
        "identifier",
        "call_expression",
        "member_expression",
        "string",
        "number",
        "true",
        "false",
        "null",
        "template_string",
        "binary_expression",
        "unary_expression",
        "parenthesized_expression",
        "comma_expression",
        "formal_parameters",
        "type_annotation",
    ]
    .iter()
    .cloned()
    .collect();

    let required_node_types: Vec<String> = pos_features[0]
        .node_types
        .iter()
        .filter(|nt| {
            !noise.contains(nt.as_str())
                && pos_features.iter().all(|f| f.node_types.contains(*nt))
                && !neg_nts.contains(&nt.as_str())
        })
        .cloned()
        .collect();

    let forbidden_node_types: Vec<String> = neg_features[0]
        .node_types
        .iter()
        .filter(|nt| {
            !noise.contains(nt.as_str())
                && neg_features.iter().all(|f| f.node_types.contains(*nt))
                && !pos_nts.contains(&nt.as_str())
        })
        .cloned()
        .collect();

    crate::corpus::semantic::LearnedConstraints {
        required_calls,
        forbidden_calls,
        required_node_types,
        forbidden_node_types,
    }
}
