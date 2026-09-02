// SPDX-License-Identifier: MIT

pub mod discovery;
pub mod events;

use glob::Pattern;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::RwLock;
use tree_sitter::{Language, Node, Query, QueryCursor, Tree};

use super::suppression::{SuppressConfig, is_suppressed};
use crate::{
    Advisory, FileId, FrensenseContext, FrensenseError, FrensenseRule, Result, TaintCache,
    parser::ParserRegistry, semantics::SymbolRegistry,
};
#[cfg(feature = "fingerprinting")]
use frensense_engine::fingerprint::{FunctionFingerprint, extract_fingerprints};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct ScanResult {
    pub advisories: Vec<Advisory>,
    #[cfg(feature = "fingerprinting")]
    pub fingerprints: Vec<FunctionFingerprint>,
}

pub struct FrensenseAuditor {
    rules: Vec<Box<dyn FrensenseRule>>,
    suppressions: Vec<(String, Pattern)>,
    rule_index: HashMap<String, usize>,
    combined_queries: std::sync::RwLock<HashMap<String, Option<Query>>>,
}

pub struct AuditOptions<'a> {
    pub file_id: FileId,
    pub path: &'a Path,
    pub content: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
    pub symbols: &'a SymbolRegistry,
    pub graph: &'a crate::semantics::graph::SemanticGraph,
    pub file_trees: &'a rustc_hash::FxHashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
    pub category_filter: &'a HashSet<String>,
    pub tag_filter: &'a HashSet<String>,
    pub suite: crate::Suite,
    pub env: crate::FrensenseEnvironment,
    pub severity_filter: Option<crate::Severity>,
    pub ngram_window_size: usize,
    pub taint_confidence_interprocedural: f64,
    pub taint_confidence_intraprocedural: f64,
    pub default_taint_max_depth: usize,
}

impl FrensenseAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn FrensenseRule>>) -> Self {
        Self {
            rule_index: Self::build_rule_index(&rules),
            rules,
            suppressions: Vec::new(),
            combined_queries: RwLock::new(HashMap::new()),
        }
    }

    fn build_rule_index(rules: &[Box<dyn FrensenseRule>]) -> HashMap<String, usize> {
        let mut idx = HashMap::new();
        for (i, r) in rules.iter().enumerate() {
            idx.insert(r.id().to_string(), i);
        }
        idx
    }

    #[must_use]
    pub fn default_auditor() -> Self {
        Self::new(Vec::new())
    }

    pub fn set_suppressions(&mut self, config: SuppressConfig) {
        for s in config.suppressions {
            if let Ok(p) = Pattern::new(&s.path) {
                self.suppressions.push((s.rule_id, p));
            }
        }
    }

    #[must_use]
    pub fn suppressions(&self) -> &[(String, Pattern)] {
        &self.suppressions
    }

    #[must_use]
    pub fn rules(&self) -> &[Box<dyn FrensenseRule>] {
        &self.rules
    }

    pub fn set_rules(&mut self, rules: Vec<Box<dyn FrensenseRule>>) {
        self.rule_index = Self::build_rule_index(&rules);
        self.rules = rules;
        self.combined_queries.write().unwrap().clear();
    }

    pub fn add_rules(&mut self, extra: Vec<Box<dyn FrensenseRule>>) {
        for r in &extra {
            self.rule_index.insert(r.id().to_string(), self.rules.len());
        }
        self.rules.extend(extra);
        self.combined_queries.write().unwrap().clear();
    }

    pub fn retain_rules<F>(&mut self, mut f: F)
    where
        F: FnMut(&Box<dyn FrensenseRule>) -> bool,
    {
        self.rules.retain(&mut f);
        self.rule_index = Self::build_rule_index(&self.rules);
        self.combined_queries.write().unwrap().clear();
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Remove the first rule with the given ID. Returns `true` if removed.
    pub fn remove_rule(&mut self, id: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.id() != id);
        let removed = self.rules.len() < before;
        if removed {
            self.rule_index = Self::build_rule_index(&self.rules);
            self.combined_queries.write().unwrap().clear();
        }
        removed
    }

    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Append a single rule.
    pub fn add_rule(&mut self, rule: Box<dyn FrensenseRule>) {
        self.rule_index
            .insert(rule.id().to_string(), self.rules.len());
        self.rules.push(rule);
        self.combined_queries.write().unwrap().clear();
    }

    fn is_rule_enabled(
        rule: &dyn FrensenseRule,
        category_filter: &std::collections::HashSet<String>,
        tag_filter: &std::collections::HashSet<String>,
        suite: crate::Suite,
        _env: crate::FrensenseEnvironment,
        severity_filter: Option<crate::Severity>,
    ) -> bool {
        let meta = rule.metadata();
        if !category_filter.is_empty() && !category_filter.contains(meta.category.as_ref()) {
            return false;
        }
        if !tag_filter.is_empty() && !meta.tags.iter().any(|t| tag_filter.contains(t.as_ref())) {
            return false;
        }
        if !meta.meets_suite(suite) {
            return false;
        }
        if let Some(threshold) = severity_filter
            && !meta.severity.meets_threshold(threshold)
        {
            return false;
        }
        true
    }

    /// Performs a security audit on a single file.
    ///
    /// # Example
    /// ```rust,ignore
    /// let options = AuditOptions {
    ///     path: &PathBuf::from("src/main.rs"),
    ///     content: "fn main() { println!(\"hello\"); }",
    ///     tree: &parser_tree,
    ///     // ... other options
    /// };
    /// let result = auditor.audit(&options)?;
    /// ```
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Currently this method always returns `Ok`, but it is marked as `Result` for potential future error conditions.
    pub fn audit(&self, opts: &AuditOptions<'_>) -> Result<ScanResult> {
        let mut advisories = Vec::new();
        #[cfg(feature = "fingerprinting")]
        let mut fingerprints = Vec::new();

        let ext = opts.path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext.is_empty() {
            return Ok(ScanResult::default());
        }

        // Phase 1: combined tree-sitter query (single AST traversal for all rules)
        self.run_combined_query(ext, opts, &mut advisories);

        // Phase 2: walk-tree fallback for any rules without queries
        if self.has_walk_rules(ext) {
            let taint_cache = TaintCache::default();
            let context = Self::build_context(opts, &taint_cache);
            for rule in &self.rules {
                if !Self::is_rule_enabled(
                    rule.as_ref(),
                    opts.category_filter,
                    opts.tag_filter,
                    opts.suite,
                    opts.env,
                    opts.severity_filter,
                ) {
                    continue;
                }
                if rule.applies_to(ext) && rule.query().is_none() {
                    self.walk_tree(
                        opts.tree.root_node(),
                        rule.as_ref(),
                        &context,
                        &mut advisories,
                    );
                }
            }
        }

        // Phase 3: file-level checks (max_file_lines, etc.)
        let taint_cache = TaintCache::default();
        let file_context = Self::build_context(opts, &taint_cache);
        for rule in &self.rules {
            if !Self::is_rule_enabled(
                rule.as_ref(),
                opts.category_filter,
                opts.tag_filter,
                opts.suite,
                opts.env,
                opts.severity_filter,
            ) {
                continue;
            }
            advisories.extend(rule.file_check(&file_context));
        }

        #[cfg(feature = "fingerprinting")]
        extract_fingerprints(
            opts.tree.root_node(),
            opts.content,
            opts.path,
            &mut fingerprints,
            opts.ngram_window_size,
        );

        Ok(ScanResult {
            advisories,
            #[cfg(feature = "fingerprinting")]
            fingerprints,
        })
    }

    fn build_context<'a>(
        opts: &'a AuditOptions<'_>,
        taint_cache: &'a TaintCache,
    ) -> FrensenseContext<'a> {
        FrensenseContext {
            file_id: opts.file_id,
            file_path: opts.path,
            source_code: opts.content,
            tree: opts.tree,
            symbols: opts.symbols,
            graph: opts.graph,
            semantic_ops: opts.semantic_ops,
            taint_cache,
            file_trees: opts.file_trees,
            file_context: frensense_engine::context::FileContext::extract(opts.path, opts.content),
            taint_confidence_interprocedural: opts.taint_confidence_interprocedural,
            taint_confidence_intraprocedural: opts.taint_confidence_intraprocedural,
            default_taint_max_depth: opts.default_taint_max_depth,
            ngram_window_size: opts.ngram_window_size,
        }
    }

    fn run_combined_query(
        &self,
        ext: &str,
        opts: &AuditOptions<'_>,
        advisories: &mut Vec<Advisory>,
    ) {
        let Ok(language) = ParserRegistry::get_language(opts.path) else {
            return;
        };

        {
            let mut cache = self.combined_queries.write().unwrap();
            if !cache.contains_key(ext) {
                let q = self.build_combined_query(ext, &language);
                cache.insert(ext.to_string(), q);
            }
        }

        let cache = self.combined_queries.read().unwrap();
        let Some(combined_query) = cache.get(ext).and_then(|q| q.as_ref()) else {
            return;
        };

        let taint_cache = TaintCache::default();
        let context = Self::build_context(opts, &taint_cache);

        let capture_names = combined_query.capture_names();
        let mut cursor = QueryCursor::new();
        let query_matches = cursor.matches(
            combined_query,
            opts.tree.root_node(),
            opts.content.as_bytes(),
        );

        let mut seen = HashSet::new();

        for m in query_matches {
            for capture in m.captures {
                let capture_name = capture_names
                    .get(capture.index as usize)
                    .copied()
                    .unwrap_or("");
                let Some((rule_id, kind)) = capture_name.split_once('.') else {
                    continue;
                };
                if kind != "node" && kind != "call" {
                    continue;
                }
                let Some(&rule_idx) = self.rule_index.get(rule_id) else {
                    continue;
                };
                let rule = &self.rules[rule_idx];

                if !Self::is_rule_enabled(
                    rule.as_ref(),
                    opts.category_filter,
                    opts.tag_filter,
                    opts.suite,
                    opts.env,
                    opts.severity_filter,
                ) {
                    continue;
                }

                let node_id = capture.node.id();
                if !seen.insert((rule_idx, node_id)) {
                    continue;
                }

                if !is_suppressed(
                    &self.suppressions,
                    capture.node,
                    rule.id(),
                    opts.content,
                    opts.path,
                ) {
                    advisories.extend(rule.check(capture.node, &context));
                }
            }
        }
    }

    fn build_combined_query(&self, ext: &str, language: &Language) -> Option<Query> {
        let mut patterns: Vec<String> = Vec::new();
        for rule in &self.rules {
            if !rule.applies_to(ext) {
                continue;
            }
            let Some(query_str) = rule.query() else {
                continue;
            };
            let rule_id = rule.id();
            let modified = query_str
                .replace("@node", &format!("@{rule_id}.node"))
                .replace("@call", &format!("@{rule_id}.call"));
            patterns.push(modified);
        }

        if patterns.is_empty() {
            None
        } else {
            let combined_str = format!("[\n{}\n]", patterns.join("\n"));
            Query::new(language, &combined_str).ok()
        }
    }

    fn has_walk_rules(&self, ext: &str) -> bool {
        self.rules
            .iter()
            .any(|r| r.applies_to(ext) && r.query().is_none())
    }

    fn walk_tree<'a>(
        &self,
        root: Node<'a>,
        rule: &dyn FrensenseRule,
        context: &FrensenseContext<'a>,
        out: &mut Vec<Advisory>,
    ) {
        let mut cursor = root.walk();
        loop {
            let node = cursor.node();

            if !is_suppressed(
                &self.suppressions,
                node,
                rule.id(),
                context.source_code,
                context.file_path,
            ) {
                out.extend(rule.check(node, context));
            }

            if cursor.goto_first_child() {
                continue;
            }

            loop {
                if cursor.goto_next_sibling() {
                    break;
                }
                if !cursor.goto_parent() {
                    return;
                }
            }
        }
    }

    /// Parses source code into a tree-sitter tree.
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Returns an error if the language is not supported or if the parser fails to initialize.
    pub fn parse_source(&self, path: &Path, content: &str) -> crate::Result<(Language, Tree)> {
        let language = ParserRegistry::get_language(path)?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language)?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| FrensenseError::ParseFailure(path.display().to_string()))?;
        Ok((language, tree))
    }
}
