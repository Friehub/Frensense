// SPDX-License-Identifier: MIT

use crate::Advisory;
use crate::engine::project::FileSnapshot;

/// Run semantic pattern detectors on a file snapshot.
pub fn find(snap: &FileSnapshot) -> Vec<Advisory> {
    let ext = snap.path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let language = frensense_engine::parser::ext_to_language(ext);
    if language == "unknown" {
        return Vec::new();
    }

    let runner = frensense_engine::semantic_patterns::PatternRunner::with_defaults();
    let findings = runner.scan_file(snap.tree.clone(), &snap.content, &snap.path.to_string_lossy(), language);

    findings
        .into_iter()
        .map(|f| {
            let severity = match f.severity.as_str() {
                "Critical" | "critical" => crate::Severity::Critical,
                "Warning" | "warning" => crate::Severity::Warning,
                _ => crate::Severity::Info,
            };

            Advisory::bare(
                &f.pattern_id,
                severity,
                snap.id,
                &snap.path,
                f.observation,
            )
            .with_confidence(f.confidence as f32)
            .with_line(crate::to_u32(f.line))
            .with_column(crate::to_u32(f.column))
            .with_impact(&f.impact)
            .with_improvement(&f.improvement)
            .with_tags(["toctou", "race-condition", "concurrency"])
        })
        .collect()
}
