use crate::semantics::reachability::ReachabilityChecker;
use crate::{Advisory, to_u32};

pub fn find(snap: &crate::engine::project::FileSnapshot) -> Vec<Advisory> {
    let checker = ReachabilityChecker::new(&snap.content);
    let dead = checker.find_dead_branches(snap.tree.root_node());

    dead.into_iter()
        .map(|(node, reason)| {
            let text = &snap.content[node.start_byte()..node.end_byte()];
            let truncated = if text.len() > 80 {
                format!("{}...", &text[..77])
            } else {
                text.to_string()
            };

            let confidence = if reason.contains("always false") || reason.contains("always true") {
                0.95
            } else {
                0.75
            };

            let replacement = if reason.contains("always false") {
                Some(String::new())
            } else {
                None
            };

            let mut adv = Advisory::bare(
                "DEAD_BRANCH",
                crate::Severity::Warning,
                snap.id,
                &snap.path,
                format!("{reason}: `{truncated}`"),
            )
            .with_confidence(confidence)
            .with_line(to_u32(node.start_position().row + 1))
            .with_column(to_u32(node.start_position().column))
            .with_bytes(to_u32(node.start_byte()), to_u32(node.end_byte()))
            .with_content(truncated)
            .with_impact("Dead code obscures intent and may hide logic errors.")
            .with_improvement("Remove dead branches or fix the condition to be dynamic.")
            .with_tags(["dead-code", "correctness"]);
            if let Some(r) = replacement {
                adv = adv.with_replacement(r);
            }
            adv
        })
        .collect()
}
