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

        let fp = fps.into_iter().next().ok_or_else(|| {
            format!("no function found in corpus file: {file_name}")
        })?;

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
    let without_ext = file_name
        .rsplitn(2, '.')
        .last()
        .unwrap_or(file_name);

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
}
