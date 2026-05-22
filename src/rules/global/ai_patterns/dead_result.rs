// SPDX-License-Identifier: MIT

use crate::{Advisory, GenSenseContext, GenSenseRule};
use tree_sitter::Node;

pub struct DeadResult;

impl GenSenseRule for DeadResult {
    fn metadata(&self) -> &crate::RuleMetadata {
        static META: std::sync::LazyLock<crate::RuleMetadata> = std::sync::LazyLock::new(|| {
            crate::RuleMetadata {
                id: "AI_DEAD_RESULT_DISCARD".into(),
                name: "Dead Result Discard".into(),
                severity: crate::Severity::Warning,
                observation: "A result is being silently discarded using 'let _ ='.".into(),
                impact: "Discarding results (especially from calls like 'std::fs::remove_file' or 'tx.commit') hides potential errors and makes debugging significantly more difficult.".into(),
                improvement: "Consider handling the result explicitly with 'match' or 'if let', or use '.expect()' if failure is truly impossible.".into(),
                tags: vec!["ai-risk".into(), "reliability".into()],
                category: "Logic".into(),
                confidence: 0.85,
            }
        });
        &META
    }

    fn applies_to(&self, ext: &str) -> bool {
        ext == "rs"
    }

    fn query(&self) -> Option<&str> {
        Some("(let_declaration) @let")
    }

    fn check<'a>(&self, node: Node<'a>, context: &GenSenseContext<'a>) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let code = &context.source_code[node.start_byte()..node.end_byte()];
        if code.starts_with("let _ =") && code.contains('(') {
            advisories.push(self.new_advisory(
                &node,
                context,
                "Silent result discard detected.".to_string(),
            ));
        }
        advisories
    }
}
