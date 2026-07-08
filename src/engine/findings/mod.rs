pub mod cross_file_taint;
pub mod semantic_patterns;
pub mod temporal_violation;

use crate::Advisory;
use crate::engine::project::FileSnapshot;

/// Shared context passed to all finding modules.
pub struct FindingContext<'a> {
    pub symbols: &'a crate::semantics::symbols::SymbolRegistry,
    pub dep_resolver: Option<&'a mut frensense_engine::deps::DependencyResolver>,
    pub data_flow_engine: Option<&'a frensense_engine::data_flow::DataFlowEngine>,
}

/// Trait for pluggable finding modules.
///
/// Each module analyzes a single snapshot and returns advisories.
/// Register modules in `registered_modules()` to add new checks
/// without modifying the runner.
pub trait FindingModule: Send + Sync {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory>;
}

struct TemporalViolation;
struct CrossFileTaint;
struct SemanticPatterns;

impl FindingModule for TemporalViolation {
    fn run(&self, _snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        Vec::new()
    }
}

impl FindingModule for CrossFileTaint {
    fn run(&self, _snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        Vec::new()
    }
}

impl FindingModule for SemanticPatterns {
    fn run(&self, _snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        Vec::new()
    }
}

/// Returns the registered finding modules in execution order.
#[must_use]
pub fn registered_modules() -> Vec<Box<dyn FindingModule>> {
    vec![
        Box::new(TemporalViolation),
        Box::new(CrossFileTaint),
        Box::new(SemanticPatterns),
    ]
}
