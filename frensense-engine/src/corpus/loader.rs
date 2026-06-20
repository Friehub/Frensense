// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::fingerprint::{FunctionFingerprint, extract_fingerprints};
use crate::corpus::semantic::SemanticFilter;

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
    let mut pairs: HashMap<String, (Vec<FunctionFingerprint>, Vec<FunctionFingerprint>)> =
        HashMap::new();

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

        let pattern_name = extract_pattern_name(file_name);
        let entry = pairs.entry(pattern_name).or_default();
        if is_positive {
            entry.0.extend(fps);
        } else {
            entry.1.extend(fps);
        }
    }

    let mut patterns = Vec::new();
    let semantic_filters = load_semantic_filters();
    for (name, (pos, neg)) in pairs {
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

        let advisory = load_sidecar_toml(corpus_dir, &name);

        if advisory.observation.is_none() {
            eprintln!("Corpus warning: pattern '{name}' has no sidecar .toml (observation/impact/improvement will be empty)");
        }

        patterns.push(CorpusPattern {
            id: name.clone(),
            positives: pos,
            negatives: neg,
            semantic_filter: semantic_filters.get(&name).cloned(),
            observation: advisory.observation,
            impact: advisory.impact,
            improvement: advisory.improvement,
        });
    }

    Ok(patterns)
}

struct AdvisoryText {
    observation: Option<String>,
    impact: Option<String>,
    improvement: Option<String>,
}

fn load_sidecar_toml(corpus_dir: &std::path::Path, pattern_name: &str) -> AdvisoryText {
    let toml_path = corpus_dir.join(format!("{pattern_name}.toml"));
    if !toml_path.exists() {
        return AdvisoryText {
            observation: None,
            impact: None,
            improvement: None,
        };
    }

    let Ok(content) = std::fs::read_to_string(&toml_path) else {
        return AdvisoryText {
            observation: None,
            impact: None,
            improvement: None,
        };
    };

    let Ok(doc) = content.parse::<toml::Table>() else {
        return AdvisoryText {
            observation: None,
            impact: None,
            improvement: None,
        };
    };

    AdvisoryText {
        observation: doc.get("observation").and_then(|v| v.as_str()).map(String::from),
        impact: doc.get("impact").and_then(|v| v.as_str()).map(String::from),
        improvement: doc.get("improvement").and_then(|v| v.as_str()).map(String::from),
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
    
    let content = possible_paths.iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .next();
    
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
            current_pattern = Some(line[1..line.len()-1].to_string());
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
        std::fs::write(dir.path().join("test_positive.py"), "def foo(): pass").unwrap();
        std::fs::write(dir.path().join("test_negative.py"), "def bar(): pass").unwrap();
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
