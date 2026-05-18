// SPDX-License-Identifier: MIT

pub mod discovery;
pub mod events;
pub mod project_auditor;
pub mod rules;
pub mod user_rules;

pub use project_auditor::ProjectAuditor;

use glob::Pattern;
use std::collections::HashSet;
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
    pub env: crate::GenSenseEnvironment,
}

impl GenSenseAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn GenSenseRule>>) -> Self {
        Self {
            rules,
            suppressions: Vec::new(),
        }
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
        self.rules = rules;
    }

    pub fn add_rules(&mut self, rules: Vec<Box<dyn GenSenseRule>>) {
        self.rules.extend(rules);
    }

    pub fn retain_rules<F>(&mut self, f: F)
    where
        F: FnMut(&Box<dyn GenSenseRule>) -> bool,
    {
        self.rules.retain(f);
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

        let Ok(language) = ParserRegistry::get_language(opts.path) else {
            return Ok(ScanResult::default());
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

        for rule in &self.rules {
            if !self.is_rule_enabled(
                rule.as_ref(),
                opts.category_filter,
                opts.tag_filter,
                opts.env,
            ) {
                continue;
            }

            let ext = opts.path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if rule.applies_to(ext) {
                if let Some(query_str) = rule.query() {
                    let Ok(query) = Query::new(&language, query_str) else {
                        continue; // Skip rule if query is invalid for this language
                    };
                    let mut cursor = QueryCursor::new();
                    let query_matches =
                        cursor.matches(&query, opts.tree.root_node(), opts.content.as_bytes());
                    for m in query_matches {
                        for capture in m.captures {
                            if !is_suppressed(
                                &self.suppressions,
                                capture.node,
                                rule.id(),
                                opts.content,
                                opts.path,
                            ) {
                                tracing::debug!(
                                    "DEBUG: Auditing rule {} on file {}",
                                    rule.id(),
                                    opts.path.display()
                                );
                                advisories.extend(rule.check(capture.node, &context));
                            }
                        }
                    }
                } else {
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
