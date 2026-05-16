// SPDX-License-Identifier: MIT

use super::Engine;
use crate::semantics::consistency::ConsistencyCheck;
use crate::{Advisory, FileId};

impl Engine {
    #[must_use] pub fn run_consistency_analysis(
        &self,
        file_id: FileId,
        path: &std::path::Path,
        _content: &str,
        _symbols: &crate::semantics::SymbolRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();

        let graph_advisories = Vec::new();
        let ast_advisories = Vec::new();

        let checker = ConsistencyCheck::new(ast_advisories, graph_advisories);
        let divergence = checker.detect_divergence();

        for missing in divergence.missing_in_graph {
            advisories.push(Advisory {
                rule_id: "CONSISTENCY_DIVERGENCE".to_string(),
                file_id,
                file_path: path.to_string_lossy().to_string(),
                severity: crate::Severity::Info,
                observation: format!("Graph Failure: Advisory '{}' found by AST walk but MISSED by Semantic Graph.", missing.rule_id),
                impact: "Analysis Integrity: The semantic graph engine is losing precision compared to legacy AST walking.".to_string(),
                improvement: "Check graph edge construction for this code pattern.".to_string(),
                line: missing.line,
                column: missing.column,
                start_byte: 0,
                end_byte: 0,
                original_content: String::new(),
                proposed_replacement: None,
                proposed_import: None,
                enclosing_symbol: missing.enclosing_symbol.clone(),
                confidence: 1.0,
                fingerprint: String::new(),
            });
        }

        advisories
    }
}
