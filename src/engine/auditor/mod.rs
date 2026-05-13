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

use super::fingerprint::{extract_fingerprints, FunctionFingerprint};
use super::suppression::{is_suppressed, SuppressConfig};
use crate::{
    parser::ParserRegistry, semantics::SymbolRegistry, Advisory, FileId, GenSenseContext,
    GenSenseError, GenSenseRule, Result, TaintCache,
};

pub type ScanResult = (Vec<Advisory>, Vec<FunctionFingerprint>);

pub struct GenSenseAuditor {
    pub rules: Vec<Box<dyn GenSenseRule>>,
    pub suppressions: Vec<(String, Pattern)>,
}

impl GenSenseAuditor {
    pub fn new(rules: Vec<Box<dyn GenSenseRule>>) -> Self {
        Self {
            rules,
            suppressions: Vec::new(),
        }
    }

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

    pub fn rules(&self) -> &[Box<dyn GenSenseRule>] {
        &self.rules
    }

    #[allow(clippy::too_many_arguments)]
    pub fn audit<'a>(
        &self,
        file_id: FileId,
        path: &'a Path,
        content: &'a str,
        tree: &'a tree_sitter::Tree,
        semantic_ops: &'a [crate::semantics::data_flow::normalization::SemanticOp],
        symbols: &'a SymbolRegistry,
        category_filter: &HashSet<String>,
        tag_filter: &HashSet<String>,
        env: crate::GenSenseEnvironment,
    ) -> Result<ScanResult> {
        let mut advisories = Vec::new();
        let mut fingerprints = Vec::new();

        let language = match ParserRegistry::get_language(path) {
            Ok(l) => l,
            Err(_) => return Ok((Vec::new(), Vec::new())),
        };

        let taint_cache = TaintCache::default();

        let context = GenSenseContext {
            file_id,
            file_path: path,
            source_code: content,
            tree,
            symbols,
            semantic_ops,
            taint_cache: &taint_cache,
        };

        for rule in &self.rules {
            if !self.is_rule_enabled(rule.as_ref(), category_filter, tag_filter, env) {
                continue;
            }

            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if rule.applies_to(ext) {
                if let Some(query_str) = rule.query() {
                    let query = Query::new(&language, query_str)
                        .map_err(|e| GenSenseError::Config(e.to_string()))?;
                    let mut cursor = QueryCursor::new();
                    let query_matches =
                        cursor.matches(&query, tree.root_node(), content.as_bytes());
                    for m in query_matches {
                        for capture in m.captures {
                            if !is_suppressed(
                                &self.suppressions,
                                capture.node,
                                rule.id(),
                                content,
                                path,
                            ) {
                                advisories.extend(rule.check(capture.node, &context));
                            }
                        }
                    }
                } else {
                    advisories.extend(self.run_recursive(
                        tree.root_node(),
                        rule.as_ref(),
                        &context,
                    ));
                }
            }
        }

        #[cfg(feature = "fingerprinting")]
        extract_fingerprints(tree.root_node(), content, path, &mut fingerprints);

        Ok((advisories, fingerprints))
    }

    pub fn run_recursive<'a>(
        &self,
        node: Node<'a>,
        rule: &dyn GenSenseRule,
        context: &GenSenseContext<'a>,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        if !is_suppressed(
            &self.suppressions,
            node,
            rule.id(),
            context.source_code,
            context.file_path,
        ) {
            advisories.extend(rule.check(node, context));
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            advisories.extend(self.run_recursive(child, rule, context));
        }

        advisories
    }

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
