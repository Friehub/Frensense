// SPDX-License-Identifier: MIT

use super::Engine;
use crate::semantics::consistency::ConsistencyCheck;
use crate::Advisory;

impl Engine {
    pub fn run_consistency_analysis(
        &self,
        path: &std::path::Path,
        _content: &str,
        _symbols: &crate::semantics::SymbolRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        // 1. Path A: Graph-based analysis
        let graph_advisories = Vec::new();
        for rule in self.auditor.rules() {
            if let Some(_fc) = rule.query() {
                // If it's a query-based rule, we can't easily compare yet
                continue;
            }

            // This is a simplification; in a real scenario we'd run specific graph-based rules
            // For now, we simulate by checking if the rule has a graph-based path
        }

        // 2. Path B: AST-based analysis (recursive)
        let ast_advisories = Vec::new();
        // ... (call auditor.run_recursive)

        // Compare
        let checker = ConsistencyCheck::new(ast_advisories, graph_advisories);
        let divergence = checker.detect_divergence();

        for missing in divergence.missing_in_graph {
            advisories.push(Advisory {
                rule_id: "CONSISTENCY_DIVERGENCE".to_string(),
                severity: crate::Severity::Info,
                observation: format!("Graph Failure: Advisory '{}' found by AST walk but MISSED by Semantic Graph.", missing.rule_id),
                impact: "Analysis Integrity: The semantic graph engine is losing precision compared to legacy AST walking.".to_string(),
                improvement: "Check graph edge construction for this code pattern.".to_string(),
                line: missing.line,
                column: missing.column,
                file_path: path.to_string_lossy().to_string(),
                original_content: String::new(),
                proposed_replacement: None,
            });
        }

        advisories
    }
}
