// SPDX-License-Identifier: MIT

use tree_sitter::Node;

use super::PatternFinding;

/// A semantic pattern detector.
///
/// Each implementation detects a specific class of semantic bug.
/// Implement this trait to add new pattern types.
///
/// # Example
///
/// ```ignore
/// struct MyPattern;
///
/// impl SemanticPattern for MyPattern {
///     fn id(&self) -> &str { "MY_PATTERN" }
///     fn description(&self) -> &str { "Detects X" }
///     fn severity(&self) -> &str { "Warning" }
///     fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding> {
///         // Walk AST and find violations
///         vec![]
///     }
/// }
/// ```
pub trait SemanticPattern: Send + Sync {
    /// Unique identifier for this pattern (e.g., "`CHECK_THEN_ACT_TOCTOU`").
    fn id(&self) -> &str;

    /// Human-readable description of what this pattern detects.
    fn description(&self) -> &str;

    /// Default severity for findings from this pattern.
    fn severity(&self) -> &str;

    /// Languages this pattern applies to ("typescript", "rust", "*" for all).
    fn languages(&self) -> &[&str] {
        &["*"]
    }

    /// Scan a single file's AST and return findings.
    fn scan(&self, tree: Node, source: &str, file_path: &str) -> Vec<PatternFinding>;
}

/// Registry of semantic patterns.
pub struct PatternRegistry {
    patterns: Vec<Box<dyn SemanticPattern>>,
}

impl Default for PatternRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternRegistry {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn register(&mut self, pattern: Box<dyn SemanticPattern>) {
        self.patterns.push(pattern);
    }

    pub fn patterns(&self) -> &[Box<dyn SemanticPattern>] {
        &self.patterns
    }

    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

/// Runs all registered patterns against source files.
pub struct PatternRunner {
    registry: PatternRegistry,
}

impl PatternRunner {
    pub fn new(registry: PatternRegistry) -> Self {
        Self { registry }
    }

    pub fn with_defaults() -> Self {
        let registry = PatternRegistry::new();
        Self { registry }
    }

    pub fn registry(&self) -> &PatternRegistry {
        &self.registry
    }

    /// Run all patterns against a parsed file.
    pub fn scan_file(
        &self,
        tree: tree_sitter::Tree,
        source: &str,
        file_path: &str,
        language: &str,
    ) -> Vec<PatternFinding> {
        let mut findings = Vec::new();
        let root = tree.root_node();

        for pattern in &self.registry.patterns {
            // Skip patterns that don't apply to this language
            if !pattern.languages().contains(&"*") && !pattern.languages().contains(&language) {
                continue;
            }

            findings.extend(pattern.scan(root, source, file_path));
        }

        findings
    }
}

impl Default for PatternRunner {
    fn default() -> Self {
        Self::with_defaults()
    }
}
