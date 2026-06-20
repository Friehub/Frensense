use crate::Advisory;
use crate::engine::project::FileSnapshot;

pub fn find(snap: &FileSnapshot) -> Vec<Advisory> {
    let mut analyzer = frensense_engine::atomic_section::AtomicSectionAnalyzer::new();
    analyzer.analyze(snap.tree.root_node(), &snap.content, &snap.path);

    let sections = analyzer.sections();
    let mut advisories = Vec::new();

    for section in sections {
        if section.is_complete {
            continue;
        }

        let op_name = match section.start_event.op {
            frensense_engine::atomic_section::AtomicOp::Lock => "lock",
            frensense_engine::atomic_section::AtomicOp::Unlock => "unlock",
            frensense_engine::atomic_section::AtomicOp::CondWait => "cond_wait",
            frensense_engine::atomic_section::AtomicOp::CondSignal => "cond_signal",
            frensense_engine::atomic_section::AtomicOp::AtomicLoad => "atomic_load",
            frensense_engine::atomic_section::AtomicOp::AtomicStore => "atomic_store",
            frensense_engine::atomic_section::AtomicOp::Fence => "fence",
        };
        let expected_op = if op_name == "lock" { "unlock" } else { "release" };

        let advisory = Advisory::bare(
            "ATOMIC_SECTION_INCOMPLETE",
            crate::Severity::Warning,
            snap.id,
            &snap.path,
            format!(
                "Incomplete lock section: {}() at line {} without matching {}()",
                op_name,
                section.start_event.line,
                expected_op
            ),
        )
        .with_confidence(0.85)
        .with_line(crate::to_u32(section.start_event.line))
        .with_column(crate::to_u32(section.start_event.column))
        .with_content(&section.lock_var)
        .with_impact("Lock acquired but never released, may cause deadlock or resource leak.")
        .with_improvement("Add explicit unlock/release call or use RAII guard pattern.")
        .with_tags(["atomic", "resource-safety"]);

        advisories.push(advisory);
    }

    advisories
}
