// [LICENSE] Proprietary - Friehub (TaaS Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

use glob::Pattern;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tree_sitter::{Node, Parser, Query, QueryCursor};
use walkdir::WalkDir;

use crate::{
    parser::ParserRegistry, rules::core::CoreRule, semantics::Symbol, semantics::SymbolRegistry,
    Advisory, GenSenseContext, GenSenseError, GenSenseRule, Result, EMBEDDED_RULES_DIR,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuppressConfig {
    pub suppressions: Vec<Suppression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suppression {
    pub rule_id: String,
    pub path: String,
}

#[cfg(feature = "fingerprinting")]
#[derive(Debug)]
pub struct FunctionFingerprint {
    pub file_path: String,
    pub function_name: String,
    pub line: usize,
    pub ngram_hashes: HashSet<u64>,
}

#[cfg(not(feature = "fingerprinting"))]
pub type FunctionFingerprint = ();

pub struct GenSenseAuditor {
    rules: Vec<Box<dyn GenSenseRule>>,
    suppressions: Vec<(String, Pattern)>,
}

impl GenSenseAuditor {
    pub fn new(rules: Vec<Box<dyn GenSenseRule>>) -> Self {
        Self {
            rules,
            suppressions: Vec::new(),
        }
    }

    pub fn set_suppressions(&mut self, config: SuppressConfig) {
        for s in config.suppressions {
            if let Ok(p) = Pattern::new(&s.path) {
                self.suppressions.push((s.rule_id, p));
            }
        }
    }

    pub fn rules(&self) -> &[Box<dyn GenSenseRule>] {
        &self.rules
    }

    /// Factory: Creates an auditor with default embedded rules.
    pub fn default_auditor() -> Self {
        let mut rules: Vec<Box<dyn GenSenseRule>> = Vec::new();

        // Native High-Precision Rules
        #[cfg(feature = "rust")]
        {
            rules.push(Box::new(crate::rules::rust::deadlock_guard::DeadlockGuard));
            rules.push(Box::new(crate::rules::rust::async_safety::AsyncPanicSafety));
            rules.push(Box::new(crate::rules::rust::fake_async::FakeAsyncDetector));
            rules.push(Box::new(
                crate::rules::rust::blocking_io::BlockingIoDetector,
            ));
            rules.push(Box::new(crate::rules::rust::tracing_guard::TracingGuard));
            rules.push(Box::new(crate::rules::rust::timeout_guard::TimeoutGuard));
        }

        // AI Patterns (Decomposed)
        rules.push(Box::new(
            crate::rules::global::ai_patterns::placeholder_panic::PlaceholderPanic,
        ));
        rules.push(Box::new(
            crate::rules::global::ai_patterns::tautological_assert::TautologicalAssert,
        ));
        rules.push(Box::new(
            crate::rules::global::ai_patterns::dead_result::DeadResult,
        ));
        rules.push(Box::new(
            crate::rules::global::ai_patterns::useless_test::UselessTest,
        ));
        rules.push(Box::new(
            crate::rules::global::ai_patterns::redundant_comment::RedundantComment,
        ));
        #[cfg(feature = "typescript")]
        rules.push(Box::new(
            crate::rules::global::ai_patterns::ts_floating_promise::TsFloatingPromiseDetector,
        ));

        rules.push(Box::new(crate::rules::global::secret_guard::SecretGuard));
        rules.push(Box::new(crate::rules::global::todo_guard::TodoGuard));

        #[derive(serde::Deserialize)]
        struct RulesWrapper {
            rules: Vec<CoreRule>,
        }

        for file in EMBEDDED_RULES_DIR.find("**/*.yml").unwrap() {
            if let Some(rules_yml) = file.as_file().and_then(|f| f.contents_utf8()) {
                if let Ok(wrapper) = serde_yaml::from_str::<RulesWrapper>(rules_yml) {
                    for rule in wrapper.rules {
                        rules.push(Box::new(rule));
                    }
                }
            }
        }

        Self::new(rules)
    }

    #[allow(unused_variables)]
    pub fn audit(
        &self,
        path: &Path,
        content: &str,
        symbols: &SymbolRegistry,
        category_filter: &HashSet<String>,
        tag_filter: &HashSet<String>,
        env: crate::GenSenseEnvironment,
    ) -> Result<(Vec<Advisory>, Vec<FunctionFingerprint>)> {
        let mut advisories = Vec::new();
        let mut fingerprints = Vec::new();

        let language = match ParserRegistry::get_language(path) {
            Ok(l) => l,
            Err(_) => return Ok((Vec::new(), Vec::new())),
        };

        let mut parser = Parser::new();
        parser.set_language(&language)?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| GenSenseError::ParseFailure(path.display().to_string()))?;

        for rule in &self.rules {
            if !self.is_rule_enabled(rule.as_ref(), category_filter, tag_filter, env) {
                continue;
            }

            if rule.applies_to(path.extension().and_then(|s| s.to_str()).unwrap_or("")) {
                let rule_advisories = if let Some(query_str) = rule.query() {
                    let query = Query::new(&language, query_str)
                        .map_err(|e| GenSenseError::Config(e.to_string()))?;
                    let mut cursor = QueryCursor::new();
                    let query_matches =
                        cursor.matches(&query, tree.root_node(), content.as_bytes());
                    let mut matches = Vec::new();
                    for m in query_matches {
                        for capture in m.captures {
                            if !self.is_suppressed(capture.node, rule.id(), content, path) {
                                matches.extend(rule.check(
                                    capture.node,
                                    &GenSenseContext {
                                        file_path: path,
                                        source_code: content,
                                        symbols,
                                    },
                                ));
                            }
                        }
                    }
                    matches
                } else {
                    if !self.is_suppressed(tree.root_node(), rule.id(), content, path) {
                        rule.check(
                            tree.root_node(),
                            &GenSenseContext {
                                file_path: path,
                                source_code: content,
                                symbols,
                            },
                        )
                    } else {
                        Vec::new()
                    }
                };

                for mut adv in rule_advisories {
                    adv.file_path = path.to_string_lossy().to_string();
                    advisories.push(adv);
                }
            }
        }

        #[cfg(feature = "fingerprinting")]
        self.extract_fingerprints(tree.root_node(), content, path, &mut fingerprints);

        Ok((advisories, fingerprints))
    }

    fn is_rule_enabled(
        &self,
        rule: &dyn GenSenseRule,
        cat_filter: &HashSet<String>,
        tag_filter: &HashSet<String>,
        env: crate::GenSenseEnvironment,
    ) -> bool {
        let tags = rule.tags();

        // --- Isolation Logic ---
        // 'beta' rules ONLY run in non-production environments
        if env == crate::GenSenseEnvironment::Production && tags.contains(&"beta") {
            return false;
        }

        if cat_filter.is_empty() && tag_filter.is_empty() {
            return true;
        }

        if !cat_filter.is_empty() && cat_filter.contains(rule.category()) {
            return true;
        }

        if !tag_filter.is_empty() {
            for tag in tags {
                if tag_filter.contains(tag) {
                    return true;
                }
            }
        }

        false
    }

    fn is_suppressed(&self, node: Node, rule_id: &str, source: &str, path: &Path) -> bool {
        // 1. Config-level suppression
        for (sid, pattern) in &self.suppressions {
            if (sid == rule_id || sid == "all") && pattern.matches_path(path) {
                return true;
            }
        }

        // 2. Inline suppression
        let start_row = node.start_position().row;
        let lines: Vec<&str> = source.lines().collect();
        let target = format!("gensense-ignore: {rule_id}");
        let target_all = "gensense-ignore: all";

        let search_start = start_row.saturating_sub(2);
        for i in search_start..start_row {
            if let Some(line) = lines.get(i) {
                if line.contains("//") && (line.contains(&target) || line.contains(target_all)) {
                    return true;
                }
            }
        }
        if let Some(line) = lines.get(start_row) {
            if line.contains("//") && (line.contains(&target) || line.contains(target_all)) {
                return true;
            }
        }

        false
    }

    #[cfg(feature = "fingerprinting")]
    fn extract_fingerprints(
        &self,
        node: Node,
        source_code: &str,
        path: &Path,
        fingerprints: &mut Vec<FunctionFingerprint>,
    ) {
        let kind = node.kind();
        if matches!(
            kind,
            "function_item" | "function_declaration" | "method_definition" | "arrow_function"
        ) {
            if let Some(body) = node.child_by_field_name("body") {
                let mut function_name = "anonymous".to_string();
                if let Some(name_node) = node.child_by_field_name("name") {
                    function_name =
                        source_code[name_node.start_byte()..name_node.end_byte()].to_string();
                } else if kind == "arrow_function" {
                    if let Some(parent) = node.parent() {
                        if parent.kind() == "variable_declarator" {
                            if let Some(name_node) = parent.child_by_field_name("name") {
                                function_name = source_code
                                    [name_node.start_byte()..name_node.end_byte()]
                                    .to_string();
                            }
                        }
                    }
                }

                let body_code = &source_code[body.start_byte()..body.end_byte()];
                let tokens: Vec<&str> = body_code
                    .split_whitespace()
                    .filter(|t| !t.is_empty() && !t.starts_with("//"))
                    .collect();

                if tokens.len() >= 5 {
                    let mut ngram_hashes = HashSet::new();
                    use std::hash::{Hash, Hasher};
                    for i in 0..=(tokens.len().saturating_sub(5)) {
                        let mut hasher = std::collections::hash_map::DefaultHasher::new();
                        tokens[i..i + 5].hash(&mut hasher);
                        ngram_hashes.insert(hasher.finish());
                    }

                    let pos = node.start_position();
                    fingerprints.push(FunctionFingerprint {
                        file_path: path.to_string_lossy().to_string(),
                        function_name,
                        line: pos.row + 1,
                        ngram_hashes,
                    });
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.extract_fingerprints(child, source_code, path, fingerprints);
        }
    }

    pub fn discover_symbols(&self, path: &Path, content: &str) -> Result<Vec<Symbol>> {
        let language = ParserRegistry::get_language(path)?;
        let query_str = match ParserRegistry::get_symbol_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        let mut parser = Parser::new();
        if parser.set_language(&language).is_err() {
            return Ok(Vec::new());
        }
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| GenSenseError::ParseFailure(path.display().to_string()))?;

        let query =
            Query::new(&language, query_str).map_err(|e| GenSenseError::Config(e.to_string()))?;
        let mut cursor = QueryCursor::new();
        let mut symbols = Vec::new();
        let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        for m in matches {
            for capture in m.captures {
                let name = &content[capture.node.start_byte()..capture.node.end_byte()];
                symbols.push(Symbol {
                    name: name.to_string(),
                    kind: crate::semantics::SymbolKind::Function, // Simplified
                    line: capture.node.start_position().row + 1,
                    column: capture.node.start_position().column + 1,
                    file_path: path.to_string_lossy().to_string(),
                });
            }
        }
        Ok(symbols)
    }
}

pub struct Engine {
    pub auditor: GenSenseAuditor,
    pub enabled_categories: HashSet<String>,
    pub enabled_tags: HashSet<String>,
    pub environment: crate::GenSenseEnvironment,
}

impl Engine {
    pub fn new(auditor: GenSenseAuditor) -> Self {
        Self {
            auditor,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            environment: crate::GenSenseEnvironment::Development,
        }
    }

    pub fn set_environment(&mut self, env: crate::GenSenseEnvironment) {
        self.environment = env;
    }

    pub fn enable_category(&mut self, cat: &str) {
        self.enabled_categories.insert(cat.to_string());
    }

    pub fn enable_tag(&mut self, tag: &str) {
        self.enabled_tags.insert(tag.to_string());
    }

    pub fn list_rules(&self) -> Vec<(&str, &str, crate::Severity)> {
        self.auditor
            .rules()
            .iter()
            .filter(|r| {
                self.auditor.is_rule_enabled(
                    r.as_ref(),
                    &self.enabled_categories,
                    &self.enabled_tags,
                    self.environment,
                )
            })
            .map(|r| (r.id(), r.description(), r.severity()))
            .collect()
    }

    pub fn run(&mut self, root: &Path) -> Result<Vec<Advisory>> {
        // Load suppression config if it exists
        let suppress_file = root.join(".gensense-suppress.yml");
        if suppress_file.exists() {
            if let Ok(content) = std::fs::read_to_string(suppress_file) {
                if let Ok(config) = serde_yaml::from_str::<SuppressConfig>(&content) {
                    self.auditor.set_suppressions(config);
                }
            }
        }

        let files = self.collect_files(root)?;
        let mut symbols = SymbolRegistry::new();

        let discovered: Result<Vec<Vec<Symbol>>> = files
            .par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(p)?;
                self.auditor.discover_symbols(p, &content)
            })
            .collect();

        for file_symbols in discovered? {
            for sym in file_symbols {
                symbols.insert(sym);
            }
        }

        let total_symbols: usize = symbols.symbols.values().map(|v| v.len()).sum();
        if total_symbols > 0 {
            eprintln!("[INFO] Semantic Discovery: Indexed {total_symbols} symbols across project.");
        }

        let results: Result<Vec<(Vec<Advisory>, Vec<FunctionFingerprint>)>> = files
            .into_par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(&p)?;
                let (mut advisories, fingerprints) = self.auditor.audit(
                    &p,
                    &content,
                    &symbols,
                    &self.enabled_categories,
                    &self.enabled_tags,
                    self.environment,
                )?;
                for adv in &mut advisories {
                    adv.file_path = p.to_string_lossy().to_string();
                }
                Ok((advisories, fingerprints))
            })
            .collect();

        let mut all_advisories = Vec::new();
        #[cfg(feature = "fingerprinting")]
        let mut all_fingerprints = Vec::new();

        for (adv, fp) in results? {
            all_advisories.extend(adv);
            #[cfg(feature = "fingerprinting")]
            all_fingerprints.extend(fp);
        }

        #[cfg(feature = "fingerprinting")]
        all_advisories.append(&mut self.post_process_ngrams(&all_fingerprints));
        all_advisories.append(&mut self.run_governance_checks(root));

        Ok(all_advisories)
    }

    fn run_governance_checks(&self, root: &Path) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let sbom_txt = root.join("sbom.txt");
        let bom_json = root.join("bom.json");
        if !sbom_txt.exists() && !bom_json.exists() {
            advisories.push(Advisory {
                rule_id: "INSTITUTIONAL_MISSING_SBOM".to_string(),
                severity: crate::Severity::Warning,
                observation: "Institutional Audit Failure: No Software Bill of Materials (SBOM) found.".to_string(),
                impact: "Audit Mandate: Every production-grade system must have a verifiable SBOM for supply chain security.".to_string(),
                improvement: "Generate an SBOM using 'cargo cyclonedx' and place it at 'bom.json'.".to_string(),
                line: 0,
                column: 0,
                file_path: "project".to_string(),
                original_content: String::new(),
                proposed_replacement: None,
            });
        }
        advisories
    }

    #[cfg(feature = "fingerprinting")]
    fn post_process_ngrams(&self, fingerprints: &[FunctionFingerprint]) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut similarity_map: HashMap<u64, Vec<usize>> = HashMap::new();
        for (idx, fp) in fingerprints.iter().enumerate() {
            for &hash in &fp.ngram_hashes {
                similarity_map.entry(hash).or_default().push(idx);
            }
        }

        let mut compared = HashSet::new();
        for (i, f1) in fingerprints.iter().enumerate() {
            let mut candidates = HashSet::new();
            for &hash in &f1.ngram_hashes {
                if let Some(indices) = similarity_map.get(&hash) {
                    for &j in indices {
                        if j > i {
                            candidates.insert(j);
                        }
                    }
                }
            }
            for j in candidates {
                if !compared.insert((i, j)) {
                    continue;
                }
                let f2 = &fingerprints[j];
                let intersection = f1.ngram_hashes.intersection(&f2.ngram_hashes).count();
                let union = f1.ngram_hashes.union(&f2.ngram_hashes).count();
                let similarity = intersection as f64 / union as f64;
                if similarity >= 0.8 {
                    advisories.push(Advisory {
                        rule_id: "REDUNDANT_BOILERPLATE".to_string(),
                        severity: crate::Severity::Warning,
                        observation: format!(
                            "Redundant Boilerplate: Block '{}' is {}% similar to '{}' in {}:{}.",
                            f1.function_name,
                            (similarity * 100.0) as u32,
                            f2.function_name,
                            f2.file_path,
                            f2.line
                        ),
                        impact:
                            "Institutional Law #4: Structural duplication dilutes the codebase."
                                .to_string(),
                        improvement: format!(
                            "Abstract common logic shared with {}.",
                            f2.function_name
                        ),
                        line: f1.line,
                        column: 0,
                        file_path: f1.file_path.clone(),
                        original_content: String::new(),
                        proposed_replacement: None,
                    });
                }
            }
        }
        advisories
    }

    fn collect_files(&self, root: &Path) -> Result<Vec<PathBuf>> {
        if root.is_file() {
            return Ok(vec![root.to_path_buf()]);
        }
        Ok(WalkDir::new(root).into_iter().filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Ok(meta) = e.metadata() {
                    if meta.len() > 1024 * 1024 {
                        eprintln!("[WARNING] Skipping large file ({} MB): {}. Parsing large files can degrade performance.", meta.len() / 1024 / 1024, e.path().display());
                        return false;
                    }
                }
                ParserRegistry::is_supported(e.path())
            })
            .map(|e| e.path().to_path_buf())
            .collect())
    }
}
