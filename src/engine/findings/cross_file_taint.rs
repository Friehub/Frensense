use crate::Advisory;
use crate::engine::findings::FindingContext;
use crate::engine::project::FileSnapshot;
use frensense_engine::data_flow::TaintOrigin;

#[must_use]
pub fn find(snap: &FileSnapshot, ctx: &FindingContext<'_>) -> Vec<Advisory> {
    let mut advisories = Vec::new();

    if let Some(resolver) = ctx.cross_file_taint.as_ref() {
        let paths = resolver.all_taint_paths(5);
        if !paths.is_empty() {
            eprintln!(
                "DEBUG CROSS_FILE_TAINT: found {} paths in {}",
                paths.len(),
                snap.path.display()
            );
        }
        for path in paths {
            // Only emit for the file where the source originated to avoid duplicates
            if path.source_file != snap.path.to_string_lossy() {
                continue;
            }

            if path.origin == TaintOrigin::UserInput || path.origin == TaintOrigin::Database {
                // List of typical sensitive sinks
                let sink = path.sink_symbol.to_lowercase();
                if sink.contains("eval")
                    || sink.contains("exec")
                    || sink.contains("query")
                    || sink.contains("send")
                    || sink.contains("json")
                {
                    let origin_str = if path.origin == TaintOrigin::UserInput {
                        "User input"
                    } else {
                        "Database record"
                    };
                    advisories.push(Advisory::bare(
                        "CROSS_FILE_TAINT",
                        crate::Severity::Critical,
                        snap.id,
                        &snap.path,
                        format!("{} flows from {} to sensitive sink {} in {}", origin_str, path.source_symbol, path.sink_symbol, path.sink_file),
                    )
                    .with_impact(format!("Unvalidated {} reaches a sensitive execution context across file boundaries.", origin_str.to_lowercase()))
                    .with_improvement("Add validation or sanitization to the source before passing data."));
                }
            }
        }
    }

    advisories
}
