// SPDX-License-Identifier: MIT

pub mod common;
pub mod discovery;
pub mod events;
pub mod project_auditor;
pub mod rules;
pub mod user_rules;

pub use project_auditor::ProjectAuditor;

use glob::Pattern;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tree_sitter::{Language, Node, Query, QueryCursor, Tree};

#[cfg(feature = "fingerprinting")]
use super::fingerprint::{FunctionFingerprint, extract_fingerprints};
use super::suppression::{SuppressConfig, is_suppressed};
use crate::{
    Advisory, FileId, GenSenseContext, GenSenseError, GenSenseRule, Result, TaintCache,
    parser::ParserRegistry, semantics::SymbolRegistry,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct ScanResult {
    pub advisories: Vec<Advisory>,
    #[cfg(feature = "fingerprinting")]
    pub fingerprints: Vec<FunctionFingerprint>,
}

pub struct GenSenseAuditor {
    rules: Vec<Box<dyn GenSenseRule>>,
    suppressions: Vec<(String, Pattern)>,
    rule_index: HashMap<String, usize>,
    combined_queries: RefCell<HashMap<String, Option<Query>>>,
}

pub struct AuditOptions<'a> {
    pub file_id: FileId,
    pub path: &'a Path,
    pub content: &'a str,
    pub tree: &'a tree_sitter::Tree,
    pub semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
    pub symbols: &'a SymbolRegistry,
    pub file_trees: &'a std::collections::HashMap<
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
    pub env: crate::GenSenseEnvironment,
}

impl GenSenseAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn GenSenseRule>>) -> Self {
        Self {
            rule_index: Self::build_rule_index(&rules),
            rules,
            suppressions: Vec::new(),
            combined_queries: RefCell::new(HashMap::new()),
        }
    }

    fn build_rule_index(rules: &[Box<dyn GenSenseRule>]) -> HashMap<String, usize> {
        let mut idx = HashMap::new();
        for (i, r) in rules.iter().enumerate() {
            idx.insert(r.id().to_string(), i);
        }
        idx
    }

    #[must_use]
    pub fn default_auditor() -> Self {
        let (rules, _) = Self::default_rules();
        Self::new(rules)
    }

    pub fn set_suppressions(&mut self, config: SuppressConfig) {
        for s in config.suppressions {
            if let Ok(p) = Pattern::new(&s.path) {
                self.suppressions.push((s.rule_id, p));
            }
        }
    }

    #[must_use]
    pub fn rules(&self) -> &[Box<dyn GenSenseRule>] {
        &self.rules
    }

    pub fn set_rules(&mut self, rules: Vec<Box<dyn GenSenseRule>>) {
        self.rule_index = Self::build_rule_index(&rules);
        self.rules = rules;
        self.combined_queries.borrow_mut().clear();
    }

    pub fn add_rules(&mut self, extra: Vec<Box<dyn GenSenseRule>>) {
        for r in &extra {
            self.rule_index.insert(r.id().to_string(), self.rules.len());
        }
        self.rules.extend(extra);
        self.combined_queries.borrow_mut().clear();
    }

    pub fn retain_rules<F>(&mut self, mut f: F)
    where
        F: FnMut(&Box<dyn GenSenseRule>) -> bool,
    {
        self.rules.retain(&mut f);
        self.rule_index = Self::build_rule_index(&self.rules);
        self.combined_queries.borrow_mut().clear();
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
            let context = GenSenseContext {
                file_id: opts.file_id,
                file_path: opts.path,
                source_code: opts.content,
                tree: opts.tree,
                symbols: opts.symbols,
                semantic_ops: opts.semantic_ops,
                taint_cache: &taint_cache,
                file_trees: opts.file_trees,
            };
            for rule in &self.rules {
                if !self.is_rule_enabled(
                    rule.as_ref(),
                    opts.category_filter,
                    opts.tag_filter,
                    opts.suite,
                    opts.env,
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

        #[cfg(feature = "fingerprinting")]
        extract_fingerprints(
            opts.tree.root_node(),
            opts.content,
            opts.path,
            &mut fingerprints,
        );

        Ok(ScanResult {
            advisories,
            #[cfg(feature = "fingerprinting")]
            fingerprints,
        })
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

        // Phase 1: ensure combined query is built (mutable borrow)
        {
            let mut cache = self.combined_queries.borrow_mut();
            if !cache.contains_key(ext) {
                let q = self.build_combined_query(ext, &language);
                cache.insert(ext.to_string(), q);
            }
        }

        // Phase 2: immutable borrow to use the query (Ref stays alive for whole traversal)
        let cache = self.combined_queries.borrow();
        let Some(combined_query) = cache.get(ext).and_then(|q| q.as_ref()) else {
            return;
        };

        let taint_cache = TaintCache::default();
        let context = GenSenseContext {
            file_id: opts.file_id,
            file_path: opts.path,
            source_code: opts.content,
            tree: opts.tree,
            symbols: opts.symbols,
            semantic_ops: opts.semantic_ops,
            taint_cache: &taint_cache,
            file_trees: opts.file_trees,
        };

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

                if !self.is_rule_enabled(
                    rule.as_ref(),
                    opts.category_filter,
                    opts.tag_filter,
                    opts.suite,
                    opts.env,
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
        rule: &dyn GenSenseRule,
        context: &GenSenseContext<'a>,
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
    /// Returns an error if the language is not supported or if the parser fails to initialize.
    pub fn parse_source(&self, path: &Path, content: &str) -> crate::Result<(Language, Tree)> {
        let language = ParserRegistry::get_language(path)?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language)?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| GenSenseError::ParseFailure(path.display().to_string()))?;
        Ok((language, tree))
    }
}
