use crate::Advisory;

pub fn find(snap: &crate::engine::project::FileSnapshot) -> Vec<Advisory> {
    let mut temporal = frensense_engine::temporal::TemporalAnalyzer::new();

    let temporal_rules = crate::temporal::load_all_temporal_rules(&[]);
    let engine_rules: Vec<frensense_engine::temporal::TemporalRuleToml> = temporal_rules
        .into_iter()
        .map(|r| frensense_engine::temporal::TemporalRuleToml {
            id: r.id,
            sequence: r.sequence,
            behavior: r.behavior,
            severity: r.severity,
            observation: r.observation,
            impact: r.impact,
            improvement: r.improvement,
            tags: r.tags,
        })
        .collect();
    temporal.add_rules_from_toml(&engine_rules);

    let violations = temporal.analyze_with_events(snap.tree.root_node(), &snap.content, &snap.path);

    violations.into_iter().map(|v| {
        let confidence = if v.event_type == frensense_engine::graph::EventType::Call { 0.85 } else { 0.70 };
        let severity = if v.event_type == frensense_engine::graph::EventType::Call {
            crate::Severity::Warning
        } else {
            crate::Severity::Info
        };

        Advisory::bare("TEMPORAL_VIOLATION", severity, snap.id, std::path::Path::new(&v.file_path), &v.label)
            .with_confidence(confidence)
            .with_line(crate::to_u32(v.line))
            .with_column(crate::to_u32(v.column))
            .with_content(&v.label)
            .with_impact("Temporal ordering violations may cause resource leaks, deadlocks, or undefined behavior.")
            .with_improvement("Ensure resources are released in all code paths, including error paths.")
            .with_tags(["temporal", "resource-safety"])
    }).collect()
}
