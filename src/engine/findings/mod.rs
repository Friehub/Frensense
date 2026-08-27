pub mod cross_file_taint;
pub mod middleware_audit;

use crate::Advisory;
use crate::engine::project::FileSnapshot;

/// Shared context passed to all finding modules.
pub struct FindingContext<'a> {
    pub symbols: &'a crate::semantics::symbols::SymbolRegistry,
    pub data_flow_engine: Option<&'a frensense_engine::data_flow::DataFlowEngine>,
    pub cross_file_taint:
        Option<&'a frensense_engine::data_flow::cross_file::CrossFileTaintResolver>,
    pub temporal_analyzer: Option<&'a frensense_engine::temporal::TemporalAnalyzer>,
    pub source_sink: &'a frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry,
}

/// Trait for pluggable finding modules.
///
/// Each module analyzes a single snapshot and returns advisories.
/// Register modules in `registered_modules()` to add new checks
/// without modifying the runner.
pub trait FindingModule: Send + Sync {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory>;
}

struct CrossFileTaint;
struct MiddlewareAudit;

impl FindingModule for CrossFileTaint {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        cross_file_taint::find(snap, ctx)
    }
}

impl FindingModule for MiddlewareAudit {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        middleware_audit::find(snap, ctx)
    }
}

/// Returns the registered finding modules in execution order.
#[must_use]
pub fn registered_modules() -> Vec<Box<dyn FindingModule>> {
    vec![
        Box::new(CrossFileTaint),
        Box::new(MiddlewareAudit),
    ]
}
