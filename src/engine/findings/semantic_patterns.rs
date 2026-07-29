use crate::Advisory;
use crate::Severity;
use crate::engine::project::FileSnapshot;
use frensense_engine::semantic_patterns::PatternRunner;

static PATTERN_RUNNER: std::sync::LazyLock<PatternRunner> =
    std::sync::LazyLock::new(PatternRunner::with_defaults);

fn parse_severity(s: &str) -> Severity {
    match s.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    }
}

#[must_use]
pub fn find(snap: &FileSnapshot) -> Vec<Advisory> {
    let ext = snap.path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let lang = match ext {
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "rs" => "rust",
        "go" => "go",
        "py" => "python",
        "java" => "java",
        "cs" => "csharp",
        _ => ext,
    };

    let findings = PATTERN_RUNNER.scan_file(
        snap.tree.clone(),
        &snap.content,
        &snap.path.to_string_lossy(),
        lang,
    );

    findings
        .into_iter()
        .map(|f| {
            Advisory::bare(
                &f.pattern_id,
                parse_severity(&f.severity),
                snap.id,
                &snap.path,
                &f.observation,
            )
            .with_confidence(f.confidence)
            .with_impact(f.impact)
            .with_improvement(f.improvement)
            .with_line(f.line as u32)
            .with_column(f.column as u32)
        })
        .collect()
}