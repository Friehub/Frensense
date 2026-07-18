use crate::Advisory;
use crate::engine::findings::FindingContext;
use crate::engine::project::FileSnapshot;

#[must_use]
pub fn find(snap: &FileSnapshot, ctx: &FindingContext<'_>) -> Vec<Advisory> {
    let mut advisories = Vec::new();

    if let Some(analyzer) = ctx.temporal_analyzer.as_ref() {
        let root = snap.tree.root_node();
        let events = frensense_engine::graph::extract_temporal_events(
            root,
            &snap.content,
            &snap.path,
            Some(analyzer.labels()),
        );
        let violations = analyzer.analyze_event_list(&events);

        for v in violations {
            advisories.push(Advisory::bare(
                "TEMPORAL_VIOLATION",
                crate::Severity::Critical,
                snap.id,
                &snap.path,
                v.label.clone(),
            )
            .with_line(v.line as u32)
            .with_impact("A required temporal execution sequence (e.g., locking without unlocking) was violated, which can cause resource leaks or deadlocks.")
            .with_improvement("Ensure that the necessary follow-up action is always performed across all control flow paths."));
        }
    }

    advisories
}
