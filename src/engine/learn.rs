// SPDX-License-Identifier: MIT

//! Pattern learning from corpus pairs.
//!
//! Given a positive (buggy) and negative (fixed) file:
//! 1. Copy them to a temp directory with proper naming
//! 2. Use AST diffing to extract what changed
//! 3. Generate pattern metadata from the diff
//! 4. Save as corpus files

use crate::engine::ast_diff::{diff_ast, extract_patterns_from_diff, PatternKind};
use std::path::Path;

/// Learn a pattern from a positive/negative pair.
pub fn learn_pattern(
    positive_path: &Path,
    negative_path: &Path,
    pattern_id: &str,
    output_dir: &Path,
) -> Result<LearnResult, String> {
    // Read source files for AST diffing
    let positive_source = std::fs::read_to_string(positive_path)
        .map_err(|e| format!("Failed to read {}: {}", positive_path.display(), e))?;
    let negative_source = std::fs::read_to_string(negative_path)
        .map_err(|e| format!("Failed to read {}: {}", negative_path.display(), e))?;

    // Perform AST diff
    let diff = diff_ast(
        &positive_source,
        &negative_source,
        &positive_path.to_string_lossy(),
        &negative_path.to_string_lossy(),
    )?;

    // Extract learned patterns from diff
    let learned_patterns = extract_patterns_from_diff(&diff);

    // Generate metadata from diff
    let metadata = generate_metadata(&diff, &learned_patterns);

    // Create output directory
    std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

    // Copy files with proper naming convention
    let pos_ext = positive_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let neg_ext = negative_path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let pos_name = format!("{}_positive.{}", pattern_id, pos_ext);
    let neg_name = format!("{}_negative.{}", pattern_id, neg_ext);

    let pos_dest = output_dir.join(&pos_name);
    let neg_dest = output_dir.join(&neg_name);

    std::fs::copy(positive_path, &pos_dest).map_err(|e| e.to_string())?;
    std::fs::copy(negative_path, &neg_dest).map_err(|e| e.to_string())?;

    // Write metadata file
    let metadata_path = output_dir.join(format!("{}.toml", pattern_id));
    let metadata_toml = generate_toml(pattern_id, &metadata, &learned_patterns);
    std::fs::write(&metadata_path, &metadata_toml).map_err(|e| e.to_string())?;

    // Load using the corpus loader to verify
    let patterns = frensense_engine::corpus::loader::load_corpus(output_dir)
        .map_err(|e| format!("Failed to load corpus: {}", e))?;

    let positive_fps: usize = patterns.iter().map(|p| p.positives.len()).sum();
    let negative_fps: usize = patterns.iter().map(|p| p.negatives.len()).sum();

    Ok(LearnResult {
        pattern_id: pattern_id.to_string(),
        positive_functions: positive_fps,
        negative_functions: negative_fps,
        positive_path: pos_dest,
        negative_path: neg_dest,
        metadata_path,
        diff_summary: generate_diff_summary(&diff),
        learned_patterns,
    })
}

/// Generate metadata from AST diff.
fn generate_metadata(diff: &crate::engine::ast_diff::AstDiff, patterns: &[LearnedPattern]) -> PatternMetadata {
    let mut metadata = PatternMetadata {
        bug_type: "unknown".to_string(),
        sanitizer: None,
        source_pattern: None,
        sink_pattern: None,
        confidence: 0.5,
    };

    // Analyze patterns to determine bug type
    for pattern in patterns {
        match pattern.kind {
            PatternKind::Sanitizer => {
                metadata.sanitizer = Some(pattern.description.clone());
                metadata.confidence = 0.8;
            }
            PatternKind::BugPattern => {
                metadata.sink_pattern = Some(pattern.description.clone());
            }
            _ => {}
        }
    }

    // Determine bug type from changes
    if !diff.modified_functions.is_empty() {
        let func = &diff.modified_functions[0];
        for change in &func.changes {
            match change.kind {
                crate::engine::ast_diff::ChangeKind::CallAdded => {
                    if change.description.contains("sanitize")
                        || change.description.contains("validate")
                        || change.description.contains("check")
                    {
                        metadata.bug_type = "missing_sanitization".to_string();
                    }
                }
                crate::engine::ast_diff::ChangeKind::CallRemoved => {
                    if change.description.contains("eval")
                        || change.description.contains("exec")
                        || change.description.contains("system")
                    {
                        metadata.bug_type = "dangerous_function_call".to_string();
                    }
                }
                _ => {}
            }
        }
    }

    metadata
}

/// Generate TOML metadata file.
fn generate_toml(pattern_id: &str, metadata: &PatternMetadata, patterns: &[LearnedPattern]) -> String {
    let mut toml = format!("# Auto-generated metadata for pattern: {}\n", pattern_id);
    toml.push_str(&format!("id = \"{}\"\n", pattern_id.to_uppercase()));
    toml.push_str(&format!("bug_type = \"{}\"\n", metadata.bug_type));
    toml.push_str(&format!("confidence = {}\n", metadata.confidence));

    if let Some(ref sanitizer) = metadata.sanitizer {
        toml.push_str(&format!("sanitizer = \"{}\"\n", sanitizer));
    }

    if let Some(ref sink) = metadata.sink_pattern {
        toml.push_str(&format!("sink = \"{}\"\n", sink));
    }

    if let Some(ref source) = metadata.source_pattern {
        toml.push_str(&format!("source = \"{}\"\n", source));
    }

    // Add learned patterns
    for pattern in patterns {
        toml.push_str("\n[[learned]]\n");
        toml.push_str(&format!("kind = \"{:?}\"\n", pattern.kind));
        toml.push_str(&format!("function = \"{}\"\n", pattern.function));
        toml.push_str(&format!("description = \"{}\"\n", pattern.description));
    }

    // Generate taint rule if applicable
    if metadata.bug_type == "missing_sanitization" || metadata.bug_type == "dangerous_function_call" {
        toml.push_str("\n# Auto-generated taint rule\n");
        toml.push_str("[[taint_rule]]\n");
        toml.push_str(&format!("id = \"TAINT_{}\"\n", pattern_id.to_uppercase()));
        toml.push_str(&format!("source = \"req\\.body|req\\.query|input|user\"\n"));
        toml.push_str(&format!("sink = \"{}\"\n", metadata.sink_pattern.as_deref().unwrap_or("eval|exec")));
        toml.push_str("severity = \"warning\"\n");
        toml.push_str(&format!("observation = \"Learned from corpus pair: {}\"\n", pattern_id));
        if let Some(ref sanitizer) = metadata.sanitizer {
            toml.push_str(&format!("improvement = \"Apply sanitizer before dangerous call: {}\"\n", sanitizer));
        } else {
            toml.push_str("improvement = \"Add input validation before dangerous operation\"\n");
        }
    }

    toml
}

/// Generate a human-readable diff summary.
fn generate_diff_summary(diff: &crate::engine::ast_diff::AstDiff) -> String {
    let mut summary = String::new();

    if diff.modified_functions.is_empty() {
        summary.push_str("No function modifications detected.\n");
    } else {
        summary.push_str(&format!(
            "{} function(s) modified:\n",
            diff.modified_functions.len()
        ));
        for func in &diff.modified_functions {
            summary.push_str(&format!(
                "  - {} (line {} → {}): {} change(s)\n",
                func.name,
                func.positive_line,
                func.negative_line,
                func.changes.len()
            ));
            for change in &func.changes {
                summary.push_str(&format!("    • {}: {}\n", change.kind, change.description));
            }
        }
    }

    if !diff.call_diffs.is_empty() {
        summary.push_str(&format!(
            "\n{} call graph difference(s):\n",
            diff.call_diffs.len()
        ));
        for call in &diff.call_diffs {
            summary.push_str(&format!(
                "  - {} → {:?} → {:?}\n",
                call.caller, call.callee_positive, call.callee_negative
            ));
        }
    }

    summary
}

#[derive(Debug)]
pub struct LearnResult {
    pub pattern_id: String,
    pub positive_functions: usize,
    pub negative_functions: usize,
    pub positive_path: std::path::PathBuf,
    pub negative_path: std::path::PathBuf,
    pub metadata_path: std::path::PathBuf,
    pub diff_summary: String,
    pub learned_patterns: Vec<LearnedPattern>,
}

#[derive(Debug, Clone)]
pub struct PatternMetadata {
    pub bug_type: String,
    pub sanitizer: Option<String>,
    pub source_pattern: Option<String>,
    pub sink_pattern: Option<String>,
    pub confidence: f64,
}

pub use crate::engine::ast_diff::LearnedPattern;

/// Load learned taint rules from a TOML file.
pub fn load_learned_taint_rules(path: &std::path::Path) -> Vec<LearnedTaintRule> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };

    let Ok(doc) = content.parse::<toml::Table>() else {
        return Vec::new();
    };

    let mut result = Vec::new();

    // Handle single table: taint_rule = { ... }
    if let Some(rule) = doc.get("taint_rule").and_then(|r| r.as_table()) {
        if let Some(id) = rule.get("id").and_then(|v| v.as_str()) {
            result.push(LearnedTaintRule {
                id: id.to_string(),
                source: rule.get("source").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                sink: rule.get("sink").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                severity: rule.get("severity").and_then(|v| v.as_str()).unwrap_or("warning").to_string(),
                observation: rule.get("observation").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                improvement: rule.get("improvement").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            });
        }
    }

    // Handle array of tables: [[taint_rule]]
    if let Some(rules) = doc.get("taint_rule").and_then(|r| r.as_array()) {
        for rule in rules {
            if let Some(rule_table) = rule.as_table() {
                if let Some(id) = rule_table.get("id").and_then(|v| v.as_str()) {
                    result.push(LearnedTaintRule {
                        id: id.to_string(),
                        source: rule_table.get("source").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        sink: rule_table.get("sink").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        severity: rule_table.get("severity").and_then(|v| v.as_str()).unwrap_or("warning").to_string(),
                        observation: rule_table.get("observation").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        improvement: rule_table.get("improvement").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    });
                }
            }
        }
    }

    result
}

#[derive(Debug, Clone)]
pub struct LearnedTaintRule {
    pub id: String,
    pub source: String,
    pub sink: String,
    pub severity: String,
    pub observation: String,
    pub improvement: String,
}

impl LearnedTaintRule {
    /// Convert to TOML format for use with --extra-taint-rules.
    pub fn to_toml(&self) -> String {
        format!(
            r#"[[rules]]
id = "{}"
source = "{}"
sink = "{}"
severity = "{}"
observation = "{}"
improvement = "{}"
"#,
            self.id, self.source, self.sink, self.severity, self.observation, self.improvement
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_taint_rules() {
        let content = "[[taint_rule]]\nid = \"TAINT_TEST\"\nsource = \"req.body\"\nsink = \"eval\"\nseverity = \"warning\"\nobservation = \"Test\"\nimprovement = \"Test\"";
        
        let temp_dir = std::env::temp_dir().join("frensense_test_taint");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        
        let path = temp_dir.join("test.toml");
        std::fs::write(&path, content).unwrap();
        
        let rules = load_learned_taint_rules(&path);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "TAINT_TEST");
        
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_learn_pattern() {
        let positive = "function handler(req) {\n    const input = req.body.query;\n    eval(input);\n}";
        let negative = "function handler(req) {\n    const input = req.body.query;\n    const clean = sanitize(input);\n    eval(clean);\n}";

        let temp_dir = std::env::temp_dir().join("frensense_test_learn");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let pos_file = temp_dir.join("test_positive.ts");
        let neg_file = temp_dir.join("test_negative.ts");
        std::fs::write(&pos_file, positive).unwrap();
        std::fs::write(&neg_file, negative).unwrap();

        let result = learn_pattern(&pos_file, &neg_file, "test_pattern", &temp_dir);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.pattern_id, "test_pattern");
        assert!(!result.diff_summary.is_empty());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
