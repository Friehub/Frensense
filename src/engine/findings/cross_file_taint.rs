use crate::Advisory;
use crate::Severity;
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

            // Must be a verified sink from the corpus registry
            let sink_suffix = path.sink_symbol.split('.').last().unwrap_or("");
            let sink_category = ctx
                .source_sink
                .is_sink(&path.sink_symbol)
                .or_else(|| ctx.source_sink.is_sink(sink_suffix));

            let Some(category) = sink_category else {
                continue;
            };

            let severity = match category {
                frensense_engine::corpus::source_sink::SinkCategory::CodeExecution
                | frensense_engine::corpus::source_sink::SinkCategory::SqlInjection
                | frensense_engine::corpus::source_sink::SinkCategory::CommandInjection => {
                    Severity::Critical
                }
                frensense_engine::corpus::source_sink::SinkCategory::StorageWrite
                | frensense_engine::corpus::source_sink::SinkCategory::Ssrf
                | frensense_engine::corpus::source_sink::SinkCategory::CredentialLeak => {
                    Severity::Warning
                }
                frensense_engine::corpus::source_sink::SinkCategory::PathTraversal
                | frensense_engine::corpus::source_sink::SinkCategory::Xss
                | frensense_engine::corpus::source_sink::SinkCategory::OpenRedirect => {
                    Severity::Warning
                }
                frensense_engine::corpus::source_sink::SinkCategory::ResponseLeak
                | frensense_engine::corpus::source_sink::SinkCategory::LogLeak => Severity::Warning,
                frensense_engine::corpus::source_sink::SinkCategory::Unknown => Severity::Info,
            };

            let origin_label = match &path.origin {
                TaintOrigin::UserInput => "User input",
                TaintOrigin::Environment => "Environment variable",
                TaintOrigin::Database => "Database record",
                TaintOrigin::Network => "Network input",
                TaintOrigin::FileSystem => "File-system data",
                TaintOrigin::Custom(s) => s.as_str(),
            };

            advisories.push(
                Advisory::bare(
                    "CROSS_FILE_TAINT",
                    severity,
                    snap.id,
                    &snap.path,
                    format!(
                        "{} flows from {} to sensitive sink {} in {} ({:?})",
                        origin_label,
                        path.source_symbol,
                        path.sink_symbol,
                        path.sink_file,
                        category,
                    ),
                )
                .with_impact(format!(
                    "Unvalidated {} reaches a sensitive {:?} execution context across file boundaries.",
                    origin_label.to_lowercase(),
                    category
                ))
                .with_improvement(
                    "Add validation or sanitization to the source before passing data.",
                ),
            );
        }
    }

    advisories
}
