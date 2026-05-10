// SPDX-License-Identifier: MIT

pub mod discovery;
pub mod events;
pub mod rules;

use glob::Pattern;
use std::collections::HashSet;
use std::path::Path;
use tree_sitter::{Language, Node, Query, QueryCursor, Tree};

use super::fingerprint::{extract_fingerprints, FunctionFingerprint};
use super::suppression::{is_suppressed, SuppressConfig};
use crate::{
    parser::ParserRegistry, semantics::SymbolRegistry, Advisory, GenSenseContext, GenSenseError,
    GenSenseRule, Result,
};

pub type CallEdges = Vec<(String, String)>;
pub type FileEdges = (std::path::PathBuf, CallEdges);
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
        Self::new(Self::default_rules())
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

    pub fn audit(
        &self,
        path: &Path,
        content: &str,
        symbols: &SymbolRegistry,
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

        let mut parser = tree_sitter::Parser::new();
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
                            if !is_suppressed(
                                &self.suppressions,
                                capture.node,
                                rule.id(),
                                content,
                                path,
                            ) {
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
        extract_fingerprints(tree.root_node(), content, path, &mut fingerprints);

        Ok((advisories, fingerprints))
    }

    pub fn run_recursive(
        &self,
        node: Node,
        rule: &dyn GenSenseRule,
        content: &str,
        path: &Path,
        symbols: &SymbolRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        if !is_suppressed(&self.suppressions, node, rule.id(), content, path) {
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

    pub fn parse_source(&self, path: &Path, content: &str) -> Result<(Language, Tree)> {
        let language = ParserRegistry::get_language(path)?;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language)
            .map_err(|e| GenSenseError::Config(e.to_string()))?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| GenSenseError::ParseFailure(path.display().to_string()))?;
        Ok((language, tree))
    }

    pub fn find_enclosing_function<'a>(&self, node: Node<'a>) -> Option<Node<'a>> {
        let mut parent = node.parent();
        while let Some(p) = parent {
            if matches!(
                p.kind(),
                "function_item"
                    | "function_declaration"
                    | "method_definition"
                    | "closure_expression"
                    | "arrow_function"
            ) {
                return Some(p);
            }
            parent = p.parent();
        }
        None
    }
}
