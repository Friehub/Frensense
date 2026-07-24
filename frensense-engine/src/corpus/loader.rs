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
    pub expected_context: Option<crate::context::FileContext>,
    pub cwe: Option<String>,
    pub cvss: Option<f32>,
    pub owasp: Option<String>,
    pub severity: Option<String>,
    pub runtime_probe: Option<String>,
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

    if !corpus_dir.exists() {
        return Err(format!("corpus directory does not exist: {}", corpus_dir.display()));
    }
    let entries = collect_corpus_files(corpus_dir);
    for path in entries {
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let is_positive = file_name.contains("_positive");
        // M1: Support _negative, _negative2, _negative3 ... for diverse negatives
        let is_negative = is_negative_file(file_name);

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
            // M4: Auto-infer expected_context from the positive file path+content — no TOML needed
            if entry.2.expected_context.is_none() {
                entry.2.expected_context =
                    Some(crate::context::FileContext::extract(&path, &source));
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

        // Priority: comment block > sidecar TOML (optional override) > synthesized
        // The sidecar TOML is NEVER required — it is only an escape hatch for edge cases.
        let toml_advisory = load_sidecar_toml(corpus_dir, &name);

        // Learn semantic constraints from positive/negative examples
        let learned = if !pos_features.is_empty() && !neg_features.is_empty() {
            // M2: Pass taint source awareness into constraint learning
            learn_from_features(&pos_features, &neg_features)
        } else {
            crate::corpus::semantic::LearnedConstraints::default()
        };

        // M3: Synthesize advisory text from learned constraints when no comment block is present
        let synthesized =
            synthesize_advisory(&name, &learned.required_calls, &learned.forbidden_calls);

        let observation = comment_advisory
            .observation
            .or(toml_advisory.observation)
            .or(synthesized.observation);
        let impact = comment_advisory
            .impact
            .or(toml_advisory.impact)
            .or(synthesized.impact);
        let improvement = comment_advisory
            .improvement
            .or(toml_advisory.improvement)
            .or(synthesized.improvement);

        // M4: Auto-context is already in comment_advisory.expected_context (set during file scan).
        // Sidecar TOML is a manual override; auto-inferred value is the fallback.
        let expected_context = toml_advisory
            .expected_context
            .or(comment_advisory.expected_context);

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
            expected_context,
            cwe: comment_advisory.cwe.or(toml_advisory.cwe),
            cvss: comment_advisory.cvss.or(toml_advisory.cvss),
            owasp: comment_advisory.owasp.or(toml_advisory.owasp),
            severity: comment_advisory.severity.or(toml_advisory.severity),
            runtime_probe: comment_advisory.runtime_probe.or(toml_advisory.runtime_probe),
        });
    }

    Ok(patterns)
}

#[derive(Debug, Clone, Default)]
struct AdvisoryText {
    observation: Option<String>,
    impact: Option<String>,
    improvement: Option<String>,
    expected_context: Option<crate::context::FileContext>,
    cwe: Option<String>,
    cvss: Option<f32>,
    owasp: Option<String>,
    severity: Option<String>,
    runtime_probe: Option<String>,
}

/// M3: Synthesize advisory text from what the AST diff already tells us.
/// Used as a fallback when no `[frensense]` comment block is present in the positive file.
fn synthesize_advisory(
    pattern_id: &str,
    required_calls: &[String],
    forbidden_calls: &[String],
) -> AdvisoryText {
    // Convert pattern_id like "ts_jwt_bypass" to a readable label
    let label = pattern_id.replace('_', " ");
    let observation = if required_calls.is_empty() {
        format!("Pattern '{label}' matches a known vulnerability shape.")
    } else {
        format!(
            "Function calls {}. This matches a known vulnerability ({label}).",
            required_calls.join(", ")
        )
    };
    let improvement = if forbidden_calls.is_empty() {
        "Review the function against the corpus positive example.".to_string()
    } else {
        format!(
            "Replace {} with {} and validate all inputs.",
            required_calls.join(" / "),
            forbidden_calls.join(" / ")
        )
    };
    AdvisoryText {
        observation: Some(observation),
        impact: None,
        improvement: Some(improvement),
        expected_context: None,
        cwe: None,
        cvss: None,
        owasp: None,
        severity: None,
        runtime_probe: None,
    }
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
                    "cwe" => result.cwe = Some(value),
                    "cvss" => result.cvss = value.parse::<f32>().ok(),
                    "owasp" => result.owasp = Some(value),
                    "severity" => result.severity = Some(value),
                    "runtime_probe" => result.runtime_probe = Some(value),
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

    let expected_context = doc
        .get("expected_context")
        .and_then(|t| t.as_table())
        .map(|t| {
            let env_str = t
                .get("environment")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let sens_str = t
                .get("sensitivity")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let env = match env_str {
                "Test" => crate::context::Environment::Test,
                "Mock" => crate::context::Environment::Mock,
                "RouteHandler" => crate::context::Environment::RouteHandler,
                "Utility" => crate::context::Environment::Utility,
                "Config" => crate::context::Environment::Config,
                _ => crate::context::Environment::Unknown,
            };

            let sens = match sens_str {
                "Low" => crate::context::DataSensitivity::Low,
                "Medium" => crate::context::DataSensitivity::Medium,
                "High" => crate::context::DataSensitivity::High,
                _ => crate::context::DataSensitivity::Unknown,
            };

            let frameworks = t
                .get("frameworks")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            crate::context::FileContext {
                environment: env,
                sensitivity: sens,
                frameworks,
            }
        });

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
        expected_context,
        cwe: doc.get("cwe").and_then(|v| v.as_str()).map(String::from),
        cvss: doc.get("cvss").and_then(|v| v.as_float().map(|f| f as f32)),
        owasp: doc.get("owasp").and_then(|v| v.as_str()).map(String::from),
        severity: doc.get("severity").and_then(|v| v.as_str()).map(String::from),
        runtime_probe: doc.get("runtime_probe").and_then(|v| v.as_str()).map(String::from),
    }
}

/// Load semantic filters from the TOML file.
pub fn load_semantic_filters() -> std::collections::HashMap<String, SemanticFilter> {
    // All semantic filters are now auto-learned from the corpus by
    // `compute_auto_filters` and stored in the FRC bundle.
    //
    // Hand-crafted filters were removed in commit 74a3a55.
    // If a pattern needs a constraint that the auto-learner doesn't
    // yet infer, add corpus positive/negative examples instead.
    //
    // The auto-learner currently infers:
    //   - contains_call_to (calls present in most positives)
    //   - contains_import (imports exclusive to a category)
    //   - excludes_call (calls in negatives but not positives) [disabled]
    //   - function_name_regex (common prefixes) [disabled]
    std::collections::HashMap::new()
}

/// Returns true if a file name represents any negative variant:
/// `_negative.ts`, `_negative2.ts`, `_negative3.ts`, etc.
/// Recursively collect all corpus files from a directory tree.
fn collect_corpus_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else { return result; };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            result.extend(collect_corpus_files(&path));
        } else if path.is_file() {
            result.push(path);
        }
    }
    result
}

fn is_negative_file(file_name: &str) -> bool {
    // Strip the extension first, then check suffix
    let stem = file_name.rsplitn(2, '.').last().unwrap_or(file_name);
    if stem.ends_with("_negative") {
        return true;
    }
    // Match _negative2, _negative3, ... _negative9
    if let Some(prefix) = stem.strip_suffix(|c: char| c.is_ascii_digit()) {
        if prefix.ends_with("_negative") {
            return true;
        }
    }
    false
}

fn extract_pattern_name(file_name: &str) -> String {
    let without_ext = file_name.rsplitn(2, '.').last().unwrap_or(file_name);

    // Positive files: just strip _positive suffix (single occurrence)
    if let Some(stripped) = without_ext.strip_suffix("_positive") {
        return stripped.to_string();
    }

    // Negative files: strip _negative, _negative2 ... _negative9 (single occurrence)
    if let Some(stripped) = without_ext.strip_suffix("_negative") {
        return stripped.to_string();
    }
    if let Some(digits) = without_ext.strip_suffix(|c: char| c.is_ascii_digit()) {
        if let Some(stripped) = digits.strip_suffix("_negative") {
            return stripped.to_string();
        }
    }

    without_ext.to_string()
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
        let source = r"/// [frensense]
/// observation: Function always returns true regardless of input.
/// impact: Malicious input passes validation unchecked.
/// improvement: Branch on input and return false for invalid values.
fn validate(input: &str) -> bool {
    true
}";
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

/// Known taint source access patterns — user-controlled input entry points.
/// If a positive example contains these and the negative does not, the pattern
/// requires taint access to match (auto-promotes to SemanticFilter.contains_call_to).
pub const TAINT_SOURCE_PATTERNS: &[&str] = &[
    "req.query",
    "req.body",
    "req.params",
    "req.headers",
    "req.cookies",
    "ctx.request",
    "ctx.query",
    "ctx.params",
    "ctx.body",
    "event.body",
    "request.body",
    "request.query",
    "process.argv",
    "c.req",
];

/// Collected features from a function node for constraint learning.
#[derive(Debug, Clone, Default)]
struct FunctionFeatures {
    calls: Vec<String>,
    node_types: Vec<String>,
    /// M2: Set if the function reads from a recognized taint source (user-controlled input)
    taint_sources: Vec<String>,
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

    // M2: Detect taint sources by scanning the raw source text of this function's span
    let func_src = &source[node.start_byte()..node.end_byte().min(source.len())];
    for &pattern in TAINT_SOURCE_PATTERNS {
        if func_src.contains(pattern) {
            features.taint_sources.push(pattern.to_string());
        }
    }
    features.taint_sources.sort();
    features.taint_sources.dedup();

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
        .flat_map(|f| f.calls.iter().map(std::string::String::as_str))
        .collect();
    let neg_calls: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.calls.iter().map(std::string::String::as_str))
        .collect();

    // Find calls in ALL positives but NOT in any negative
    let mut required_calls: Vec<String> = pos_features[0]
        .calls
        .iter()
        .filter(|call| {
            pos_features.iter().all(|f| f.calls.contains(*call))
                && !neg_calls.contains(&call.as_str())
        })
        .cloned()
        .collect();

    // M2: Auto-promote taint sources to required_calls when positives have taint
    // and negatives do not — eliminates FP on non-user-controlled code paths.
    let pos_has_taint = pos_features.iter().any(|f| !f.taint_sources.is_empty());
    let neg_has_taint = neg_features.iter().any(|f| !f.taint_sources.is_empty());
    if pos_has_taint && !neg_has_taint {
        // Collect taint sources present in any positive but absent from all negatives
        let neg_taint: std::collections::HashSet<&str> = neg_features
            .iter()
            .flat_map(|f| f.taint_sources.iter().map(std::string::String::as_str))
            .collect();
        for f in pos_features {
            for src in &f.taint_sources {
                if !neg_taint.contains(src.as_str()) && !required_calls.contains(src) {
                    required_calls.push(src.clone());
                }
            }
        }
    }

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
        .flat_map(|f| f.node_types.iter().map(std::string::String::as_str))
        .collect();
    let neg_nts: Vec<&str> = neg_features
        .iter()
        .flat_map(|f| f.node_types.iter().map(std::string::String::as_str))
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
    .copied()
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
        required_taint_flows: Vec::new(),
    }
}
