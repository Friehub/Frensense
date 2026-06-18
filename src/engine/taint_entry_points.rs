// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TaintEntryPoint {
    pub language: String,
    pub type_pattern: Option<String>,
    #[serde(default)]
    pub param_index: Option<usize>,
    #[serde(default)]
    pub rule_ids: Vec<String>,
}

static ENTRY_POINTS_FILE: &str = include_str!("../../taint_entry_points.toml");

static BUILTIN_ENTRY_POINTS: LazyLock<Vec<TaintEntryPoint>> =
    LazyLock::new(|| load_entry_points_from_str(ENTRY_POINTS_FILE));

pub fn load_entry_points_from_str(content: &str) -> Vec<TaintEntryPoint> {
    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(entries) = doc.get("entry_points").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .filter_map(|r| r.clone().try_into().ok())
        .collect()
}

pub fn load_entry_points_from_file(path: &Path) -> Vec<TaintEntryPoint> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    load_entry_points_from_str(&content)
}

pub fn load_all_entry_points(extra_dirs: &[PathBuf]) -> Vec<TaintEntryPoint> {
    let mut points = BUILTIN_ENTRY_POINTS.clone();

    for dir in extra_dirs {
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    points.extend(load_entry_points_from_file(&path));
                }
            }
        }
    }

    points
}

// --- Sanitizers ---

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SanitizerRule {
    pub language: String,
    pub functions: Vec<String>,
}

static SANITIZERS_FILE: &str = include_str!("../../sanitizers.toml");

static BUILTIN_SANITIZERS: LazyLock<Vec<SanitizerRule>> =
    LazyLock::new(|| load_sanitizers_from_str(SANITIZERS_FILE));

fn load_sanitizers_from_str(content: &str) -> Vec<SanitizerRule> {
    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(entries) = doc.get("sanitizers").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    entries
        .iter()
        .filter_map(|r| r.clone().try_into().ok())
        .collect()
}

pub fn build_sanitizer_regex(language: &str) -> Option<regex::Regex> {
    let all = BUILTIN_SANITIZERS.clone();
    let funcs: Vec<&str> = all
        .iter()
        .filter(|r| r.language == language)
        .flat_map(|r| r.functions.iter().map(|s| s.as_str()))
        .collect();
    if funcs.is_empty() {
        return None;
    }
    regex::Regex::new(&funcs.join("|")).ok()
}

pub fn build_sanitizer_regex_from_rules(
    rules: &[SanitizerRule],
    language: &str,
) -> Option<regex::Regex> {
    let funcs: Vec<&str> = rules
        .iter()
        .filter(|r| r.language == language)
        .flat_map(|r| r.functions.iter().map(|s| s.as_str()))
        .collect();
    if funcs.is_empty() {
        return None;
    }
    regex::Regex::new(&funcs.join("|")).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_str() {
        let toml = r#"
[[entry_points]]
language = "rust"
type_pattern = "Json|Query|Form"
rule_ids = []

[[entry_points]]
language = "typescript"
type_pattern = "Request"
param_index = 0
rule_ids = ["TAINT_INPUT_TO_HTTP"]
"#;
        let points = load_entry_points_from_str(toml);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].language, "rust");
        assert_eq!(points[0].type_pattern.as_deref(), Some("Json|Query|Form"));
        assert!(points[0].param_index.is_none());
        assert!(points[0].rule_ids.is_empty());
        assert_eq!(points[1].language, "typescript");
        assert_eq!(points[1].param_index, Some(0));
        assert_eq!(points[1].rule_ids, vec!["TAINT_INPUT_TO_HTTP"]);
    }

    #[test]
    fn test_load_empty() {
        let points = load_entry_points_from_str("");
        assert!(points.is_empty());
    }

    #[test]
    fn test_builtin_loads() {
        let points = BUILTIN_ENTRY_POINTS.clone();
        assert!(
            points.len() >= 3,
            "should have at least rust + ts + python entries"
        );
        assert!(points.iter().any(|p| p.language == "rust"));
        assert!(points.iter().any(|p| p.language == "typescript"));
    }

    #[test]
    fn test_sanitizer_regex_rust() {
        let re = build_sanitizer_regex("rust");
        assert!(re.is_some());
        let re = re.unwrap();
        assert!(re.is_match("validate"));
        assert!(re.is_match("sanitize"));
        assert!(re.is_match("from_str"));
        assert!(!re.is_match("fetch"));
    }

    #[test]
    fn test_sanitizer_regex_typescript() {
        let re = build_sanitizer_regex("typescript");
        assert!(re.is_some());
        let re = re.unwrap();
        assert!(re.is_match("encodeURIComponent"));
        assert!(re.is_match("z.parse"));
        assert!(!re.is_match("eval"));
    }

    #[test]
    fn test_sanitizer_regex_unknown_language() {
        let re = build_sanitizer_regex("go");
        assert!(re.is_none());
    }
}
