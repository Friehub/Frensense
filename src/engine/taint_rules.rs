// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TaintRuleToml {
    pub id: String,
    pub source: String,
    pub sink: String,
    pub severity: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub source_functions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TaintRule {
    pub id: String,
    pub source_re: String,
    pub sink_re: String,
    pub severity: crate::Severity,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub languages: Vec<String>,
    pub source_functions: Vec<String>,
}

impl TaintRule {
    pub fn from_toml(t: TaintRuleToml) -> Self {
        Self {
            id: t.id,
            source_re: t.source,
            sink_re: t.sink,
            severity: match t.severity.to_lowercase().as_str() {
                "critical" => crate::Severity::Critical,
                "warning" => crate::Severity::Warning,
                _ => crate::Severity::Info,
            },
            observation: t.observation,
            impact: t.impact,
            improvement: t.improvement,
            languages: t.languages,
            source_functions: t.source_functions,
        }
    }

    /// Returns true if this rule applies to the given language.
    /// Empty languages list means the rule applies to all languages.
    pub fn applies_to_language(&self, language: &str) -> bool {
        self.languages.is_empty() || self.languages.iter().any(|l| l == language)
    }
}

/// Load taint rules from a TOML file.
pub fn load_taint_rules_from_file(path: &Path) -> Vec<TaintRule> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(rules) = doc.get("rules").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    rules
        .iter()
        .filter_map(|r| {
            let t: TaintRuleToml = r.clone().try_into().ok()?;
            Some(TaintRule::from_toml(t))
        })
        .collect()
}

/// Load all taint rules: built-in TOML + user directories.
pub fn load_all_taint_rules(extra_dirs: &[PathBuf]) -> Vec<TaintRule> {
    let mut rules = BUILTIN_RULES.clone();

    // Load from extra directories
    for dir in extra_dirs {
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    rules.extend(load_taint_rules_from_file(&path));
                }
            }
        }
    }

    rules
}

static BUILTIN_RULES_FILE: &str = include_str!("../../taint_rules.toml");

static BUILTIN_RULES: LazyLock<Vec<TaintRule>> =
    LazyLock::new(|| load_taint_rules_from_str(BUILTIN_RULES_FILE));

fn load_taint_rules_from_str(content: &str) -> Vec<TaintRule> {
    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(rules) = doc.get("rules").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    rules
        .iter()
        .filter_map(|r| {
            let t: TaintRuleToml = r.clone().try_into().ok()?;
            Some(TaintRule::from_toml(t))
        })
        .collect()
}

/// Combined source regex built once from built-in taint rules.
/// For full rule set including user rules, use `build_combined_regexes`.
pub static COMBINED_SOURCE_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    let patterns: Vec<&str> = BUILTIN_RULES.iter().map(|r| r.source_re.as_str()).collect();
    regex::Regex::new(&patterns.join("|")).expect("valid combined source regex")
});

/// Combined sink regex built once from built-in taint rules.
/// For full rule set including user rules, use `build_combined_regexes`.
pub static COMBINED_SINK_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    let patterns: Vec<&str> = BUILTIN_RULES.iter().map(|r| r.sink_re.as_str()).collect();
    regex::Regex::new(&patterns.join("|")).expect("valid combined sink regex")
});

/// Build combined source/sink regexes from a full rule list (builtin + user rules).
pub fn build_combined_regexes(rules: &[TaintRule]) -> (regex::Regex, regex::Regex) {
    let source_patterns: Vec<&str> = rules.iter().map(|r| r.source_re.as_str()).collect();
    let sink_patterns: Vec<&str> = rules.iter().map(|r| r.sink_re.as_str()).collect();
    let source =
        regex::Regex::new(&source_patterns.join("|")).expect("valid combined source regex");
    let sink = regex::Regex::new(&sink_patterns.join("|")).expect("valid combined sink regex");
    (source, sink)
}

/// Legacy function — returns built-in rules only. Use `load_all_taint_rules` for full set.
pub fn security_taint_rules() -> Vec<TaintRule> {
    BUILTIN_RULES.clone()
}
