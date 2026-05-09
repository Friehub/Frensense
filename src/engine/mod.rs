// SPDX-License-Identifier: MIT

use glob::Pattern;
use petgraph::graph::NodeIndex;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tree_sitter::{Language, Node, Parser, Query, QueryCursor, Tree};
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

        fn collect_yml_files<'a>(
            dir: &'a include_dir::Dir<'a>,
            files: &mut Vec<include_dir::File<'a>>,
        ) {
            for entry in dir.entries() {
                match entry {
                    include_dir::DirEntry::Dir(d) => collect_yml_files(d, files),
                    include_dir::DirEntry::File(f) => {
                        if f.path().extension().and_then(|s| s.to_str()) == Some("yml") {
                            files.push(f.clone());
                        }
                    }
                }
            }
        }

        let mut rule_files = Vec::new();
        collect_yml_files(&EMBEDDED_RULES_DIR, &mut rule_files);

        for file in rule_files {
            if let Some(rules_yml) = file.contents_utf8() {
                match serde_yaml::from_str::<RulesWrapper>(rules_yml) {
                    Ok(wrapper) => {
                        for rule in wrapper.rules {
                            let compiled = crate::rules::compiler::RuleCompiler::compile(rule);
                            rules.push(Box::new(compiled));
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "[ERROR] Failed to parse YAML rules in {:?}: {}",
                            file.path(),
                            e
                        );
                    }
                }
            }
        }

        let _count = rules.len();
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

            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if rule.applies_to(ext) {
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
                    self.run_recursive(tree.root_node(), rule.as_ref(), content, path, symbols)
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

    fn parse_source(&self, path: &Path, content: &str) -> Result<(Language, Tree)> {
        let language = ParserRegistry::get_language(path)?;
        let mut parser = Parser::new();
        parser
            .set_language(&language)
            .map_err(|e| GenSenseError::Config(e.to_string()))?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| GenSenseError::ParseFailure(path.display().to_string()))?;
        Ok((language, tree))
    }

    fn find_enclosing_function<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        let mut parent = node.parent();
        while let Some(p) = parent {
            if matches!(
                p.kind(),
                "function_item" | "function_declaration" | "method_definition"
            ) {
                return Some(p);
            }
            parent = p.parent();
        }
        None
    }

    pub fn discover_symbols(&self, path: &Path, content: &str) -> Result<Vec<Symbol>> {
        let (_, tree) = self.parse_source(path, content)?;
        let query_str = match ParserRegistry::get_symbol_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        let language = ParserRegistry::get_language(path)?;
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
                    kind: crate::semantics::SymbolKind::Function,
                    line: capture.node.start_position().row + 1,
                    column: capture.node.start_position().column + 1,
                    file_path: path.to_string_lossy().to_string(),
                });
            }
        }
        Ok(symbols)
    }

    pub fn scan_for_edges(&self, path: &Path, content: &str) -> Result<Vec<(String, String)>> {
        let (language, tree) = self.parse_source(path, content)?;
        let query_str = match ParserRegistry::get_call_query(path) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        let query =
            Query::new(&language, query_str).map_err(|e| GenSenseError::Config(e.to_string()))?;
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        let mut edges = Vec::new();
        for m in matches {
            for capture in m.captures {
                let call_node = capture.node;
                let call_name = &content[call_node.start_byte()..call_node.end_byte()];

                if let Some(p) = self.find_enclosing_function(call_node) {
                    if let Some(name_node) = p.child_by_field_name("name") {
                        let caller_name = &content[name_node.start_byte()..name_node.end_byte()];
                        edges.push((caller_name.to_string(), call_name.to_string()));
                    }
                }
            }
        }
        Ok(edges)
    }

    fn run_recursive(
        &self,
        node: Node,
        rule: &dyn GenSenseRule,
        content: &str,
        path: &Path,
        symbols: &SymbolRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        if !self.is_suppressed(node, rule.id(), content, path) {
            advisories.extend(rule.check(
                node,
                &GenSenseContext {
                    file_path: path,
                    source_code: content,
                    symbols,
                },
            ));
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            advisories.extend(self.run_recursive(child, rule, content, path, symbols));
        }
        advisories
    }

    pub fn discover_events(
        &self,
        path: &Path,
        content: &str,
        registry: &mut SymbolRegistry,
    ) -> Result<()> {
        let (_, tree) = self.parse_source(path, content)?;
        let mut cursor = tree.walk();
        self.traverse_for_events(tree.root_node(), &mut cursor, path, content, registry, None);
        Ok(())
    }

    fn traverse_for_events<'a>(
        &self,
        node: Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        path: &Path,
        content: &str,
        registry: &mut SymbolRegistry,
        last_event: Option<NodeIndex>,
    ) -> Option<NodeIndex> {
        let mut current_last = last_event;

        // Check if this node is an event
        let (event_type, label) = match node.kind() {
            "call_expression" => {
                let fn_node = node.child_by_field_name("function");
                if let Some(f) = fn_node {
                    let fn_name = &content[f.start_byte()..f.end_byte()];
                    let et = if fn_name.contains("lock") {
                        crate::semantics::graph::EventType::Acquire
                    } else if fn_name.contains("unlock") || fn_name.contains("drop") {
                        crate::semantics::graph::EventType::Release
                    } else {
                        crate::semantics::graph::EventType::Call
                    };
                    (Some(et), fn_name.to_string())
                } else {
                    (None, String::new())
                }
            }
            "await_expression" => (
                Some(crate::semantics::graph::EventType::Await),
                ".await".to_string(),
            ),
            _ => (None, String::new()),
        };

        if let Some(et) = event_type {
            let event = crate::semantics::graph::TemporalEvent {
                event_type: et,
                label,
                file_path: path.to_string_lossy().to_string(),
                line: node.start_position().row + 1,
                column: node.start_position().column + 1,
            };
            let idx = registry.graph.add_event(event);

            // Link to previous event in sequence
            if let Some(prev) = last_event {
                registry.graph.add_edge(
                    prev,
                    idx,
                    crate::semantics::graph::EdgeKind::SequentiallyFollows,
                );
            }

            // Link to enclosing function
            if let Some(func) = self.find_enclosing_function(node) {
                if let Some(name_node) = func.child_by_field_name("name") {
                    let name = &content[name_node.start_byte()..name_node.end_byte()];
                    for &func_idx in &registry.graph.find_nodes(name) {
                        if let Some(sym) = registry.graph.get_symbol(func_idx) {
                            if sym.file_path == path.to_string_lossy() {
                                registry.graph.add_edge(
                                    func_idx,
                                    idx,
                                    crate::semantics::graph::EdgeKind::InScope,
                                );
                            }
                        }
                    }
                }
            }
            current_last = Some(idx);
        }

        if cursor.goto_first_child() {
            loop {
                current_last = self.traverse_for_events(
                    cursor.node(),
                    cursor,
                    path,
                    content,
                    registry,
                    current_last,
                );
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }

        current_last
    }
}

pub struct Engine {
    pub auditor: GenSenseAuditor,
    pub enabled_categories: HashSet<String>,
    pub enabled_tags: HashSet<String>,
    pub environment: crate::GenSenseEnvironment,
    pub verify_consistency: bool,
}

impl Engine {
    pub fn new(auditor: GenSenseAuditor) -> Self {
        Self {
            auditor,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            environment: crate::GenSenseEnvironment::Development,
            verify_consistency: false,
        }
    }

    pub fn set_consistency_verification(&mut self, enabled: bool) {
        self.verify_consistency = enabled;
    }

    pub fn set_environment(&mut self, env: crate::GenSenseEnvironment) {
        self.environment = env;
    }

    pub fn enable_category(&mut self, cat: &str) {
        self.enabled_categories.insert(cat.to_string());
    }

    #[allow(dead_code)]
    fn enable_tag_internal(&mut self, tag: &str) {
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
        let (advisories, _) = self.run_detailed(root)?;
        Ok(advisories)
    }

    pub fn run_detailed(&mut self, root: &Path) -> Result<(Vec<Advisory>, SymbolRegistry)> {
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

        // Pass 2: Edge Discovery (Inter-procedural linking) - PARALLEL
        let edge_results: Result<Vec<(PathBuf, Vec<(String, String)>)>> = files
            .par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(p)?;
                let edges = self.auditor.scan_for_edges(p, &content)?;
                Ok((p.clone(), edges))
            })
            .collect();

        for (path, file_edges) in edge_results? {
            for (caller, callee) in file_edges {
                symbols.add_call_edge(&path, &caller, &callee);
            }
        }

        // Pass 3: Event Discovery (Temporal timelines)
        for p in &files {
            let content = std::fs::read_to_string(p)?;
            self.auditor.discover_events(p, &content, &mut symbols)?;
        }

        let total_symbols = symbols.graph.graph.node_count();
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
        if self.enabled_tags.contains("governance") || self.enabled_tags.contains("sbom") {
            all_advisories.append(&mut self.run_governance_checks(root));
        }

        Ok((all_advisories, symbols))
    }

    fn run_governance_checks(&self, root: &Path) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let sbom_txt = root.join("sbom.txt");
        let bom_json = root.join("bom.json");
        if !sbom_txt.exists() && !bom_json.exists() {
            advisories.push(Advisory {
                rule_id: "MISSING_SBOM".to_string(),
                severity: crate::Severity::Warning,
                observation: "Project Health: No Software Bill of Materials (SBOM) found.".to_string(),
                impact: "Supply Chain Security: A verifiable SBOM is recommended for production-grade systems to track dependencies.".to_string(),
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
                            "Engineering Principle: Structural duplication increases technical debt and maintenance overhead."
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

    pub fn run_content(&self, file_path: &Path, content: &str) -> Result<Vec<Advisory>> {
        let mut symbols = crate::semantics::SymbolRegistry::new();
        let discovered = self.auditor.discover_symbols(file_path, content)?;
        for sym in discovered {
            symbols.insert(sym);
        }

        let (advisories, _) = self.auditor.audit(
            file_path,
            content,
            &symbols,
            &self.enabled_categories,
            &self.enabled_tags,
            self.environment,
        )?;
        Ok(advisories)
    }

    pub fn enable_tag(&mut self, tag: &str) {
        self.enabled_tags.insert(tag.to_string());
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
