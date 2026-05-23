use crate::{Advisory, GenSenseContext, GenSenseRule, RuleMetadata};

const MAX_SOURCE_LINES: usize = 500;

pub struct LongFile;

impl GenSenseRule for LongFile {
    fn metadata(&self) -> &RuleMetadata {
        static META: std::sync::LazyLock<RuleMetadata> = std::sync::LazyLock::new(|| {
            RuleMetadata {
                id: "LONG_FILE".into(),
                name: "File Exceeds Line Limit".into(),
                severity: crate::Severity::Warning,
                observation: "Source file exceeds the recommended maximum length.".into(),
                impact: "Oversized files accumulate mixed concerns, reduce maintainability, and are reliably correlated with defect density.".into(),
                improvement: "Split the file into smaller modules by concern (e.g., one type per file, one service per module).".into(),
                tags: vec!["quality".into(), "maintainability".into()],
                category: "Quality".into(),
                confidence: 0.85,
                precision: crate::Precision::VeryHigh,
            }
        });
        &META
    }

    fn applies_to(&self, _ext: &str) -> bool {
        true
    }

    fn query(&self) -> Option<&str> {
        None
    }

    fn check<'a>(
        &self,
        _node: tree_sitter::Node<'a>,
        _context: &GenSenseContext<'a>,
    ) -> Vec<Advisory> {
        Vec::new()
    }

    fn file_check(&self, context: &GenSenseContext<'_>) -> Vec<Advisory> {
        let line_count = context.source_code.lines().count();
        if line_count > MAX_SOURCE_LINES {
            let meta = self.metadata();
            return vec![Advisory {
                rule_id: meta.id.to_string(),
                file_id: context.file_id,
                file_path: context.file_path.to_string_lossy().to_string(),
                severity: meta.severity,
                confidence: meta.confidence,
                observation: format!(
                    "File length ({line_count} lines) exceeds threshold of {MAX_SOURCE_LINES}."
                ),
                impact: meta.impact.to_string(),
                improvement: meta.improvement.to_string(),
                line: 1,
                column: 1,
                start_byte: 0,
                end_byte: 0,
                original_content: String::new(),
                proposed_replacement: None,
                proposed_import: None,
                enclosing_symbol: None,
                fingerprint: String::new(),
                auto_fixable: false,
                requires_human: false,
                tags: meta.tags.iter().map(ToString::to_string).collect(),
            }];
        }
        Vec::new()
    }
}
