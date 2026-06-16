// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::fingerprint::{FunctionFingerprint, extract_fingerprints};

#[derive(Debug, Clone)]
pub struct CorpusPattern {
    pub id: String,
    pub positive: FunctionFingerprint,
    pub negative: FunctionFingerprint,
}

pub fn load_corpus(corpus_dir: &Path) -> Result<Vec<CorpusPattern>, String> {
    let mut pairs: HashMap<String, (Option<FunctionFingerprint>, Option<FunctionFingerprint>)> =
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

        let mut parser = tree_sitter::Parser::new();
        let lang = crate::parser::ParserRegistry::get_language_by_name(match ext {
            "rs" => "rust",
            "ts" | "tsx" => "typescript",
            "js" | "jsx" => "javascript",
            _ => continue,
        })
        .map_err(|e| e.to_string())?;
        parser.set_language(&lang).map_err(|e| e.to_string())?;
        let Some(tree) = parser.parse(&source, None) else {
            continue;
        };

        let mut fps = Vec::new();
        extract_fingerprints(tree.root_node(), &source, &path, &mut fps, 5);

        let Some(fp) = fps.into_iter().next() else {
            continue; // Silently skip files with no parseable function
        };

        let pattern_name = extract_pattern_name(file_name);
        let entry = pairs.entry(pattern_name).or_default();
        if is_positive {
            entry.0 = Some(fp);
        } else {
            entry.1 = Some(fp);
        }
    }

    let mut patterns = Vec::new();
    for (name, (pos, neg)) in pairs {
        match (pos, neg) {
            (Some(p), Some(n)) => patterns.push(CorpusPattern {
                id: name,
                positive: p,
                negative: n,
            }),
            (Some(_p), None) => {
                eprintln!("Corpus warning: pattern '{name}' has positive but no negative example");
            }
            (None, Some(_n)) => {
                eprintln!("Corpus warning: pattern '{name}' has negative but no positive example");
            }
            (None, None) => unreachable!(),
        }
    }

    Ok(patterns)
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
    }
}
