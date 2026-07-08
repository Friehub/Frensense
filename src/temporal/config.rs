// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct TemporalRuleToml {
    pub id: String,
    pub sequence: Vec<String>,
    pub behavior: String,
    pub severity: String,
    pub observation: String,
    pub impact: String,
    pub improvement: String,
    pub tags: Vec<String>,
}

static BUILTIN_TEMPORAL_RULES_FILE: &str = include_str!("../../temporal_rules.toml");

static BUILTIN_TEMPORAL_RULES: LazyLock<Vec<TemporalRuleToml>> =
    LazyLock::new(|| load_temporal_rules_from_str(BUILTIN_TEMPORAL_RULES_FILE));

fn parse_rule_from_table(table: &toml::Value) -> Option<TemporalRuleToml> {
    let id = table.get("id")?.as_str()?.to_string();
    let sequence = table
        .get("sequence")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    let behavior = table.get("behavior")?.as_str()?.to_string();
    let severity = table.get("severity")?.as_str()?.to_string();
    let observation = table.get("observation")?.as_str()?.to_string();
    let impact = table.get("impact")?.as_str()?.to_string();
    let improvement = table.get("improvement")?.as_str()?.to_string();
    let tags = table
        .get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Some(TemporalRuleToml {
        id,
        sequence,
        behavior,
        severity,
        observation,
        impact,
        improvement,
        tags,
    })
}

fn load_temporal_rules_from_str(content: &str) -> Vec<TemporalRuleToml> {
    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };
    let Some(rules) = doc.get("rules").and_then(|r| r.as_array()) else {
        return Vec::new();
    };
    rules.iter().filter_map(parse_rule_from_table).collect()
}

#[must_use]
pub fn load_temporal_rules_from_file(path: &Path) -> Vec<TemporalRuleToml> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    load_temporal_rules_from_str(&content)
}

pub fn load_all_temporal_rules(extra_dirs: &[PathBuf]) -> Vec<TemporalRuleToml> {
    let mut rules = BUILTIN_TEMPORAL_RULES.clone();

    for dir in extra_dirs {
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    rules.extend(load_temporal_rules_from_file(&path));
                }
            }
        }
    }

    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_builtin_temporal_rules() {
        let rules = load_all_temporal_rules(&[]);
        // Temporal rules are now corpus-driven — no hardcoded rules remain
        assert_eq!(rules.len(), 0);
    }

    #[test]
    fn test_load_from_str() {
        let toml = r#"
[[rules]]
id = "test_rule"
sequence = ["a", "b"]
behavior = "must_follow"
severity = "warning"
observation = "Test observation"
impact = "Test impact"
improvement = "Test improvement"
"#;
        let rules = load_temporal_rules_from_str(toml);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "test_rule");
    }
}
