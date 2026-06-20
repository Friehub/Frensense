pub mod atomic_section;
pub mod cross_file_taint;
pub mod dead_branch;
pub mod hallucinated_import;
pub mod semantic_patterns;
pub mod temporal_violation;
pub mod unused_variable;

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

struct DeadBranch;
struct UnusedVariable;
struct TemporalViolation;
struct HallucinatedImport;
struct CrossFileTaint;
struct AtomicSection;
struct SemanticPatterns;

impl FindingModule for DeadBranch {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        dead_branch::find(snap)
    }
}

impl FindingModule for UnusedVariable {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        unused_variable::find(snap)
    }
}

impl FindingModule for TemporalViolation {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        temporal_violation::find(snap)
    }
}

impl FindingModule for HallucinatedImport {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        if let Some(ref mut resolver) = ctx.dep_resolver {
            hallucinated_import::find(resolver, snap)
        } else {
            Vec::new()
        }
    }
}

impl FindingModule for CrossFileTaint {
    fn run(&self, snap: &FileSnapshot, ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        let data_flow = ctx.data_flow_engine;
        cross_file_taint::find(ctx.symbols, snap, data_flow)
    }
}

impl FindingModule for AtomicSection {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        atomic_section::find(snap)
    }
}

impl FindingModule for SemanticPatterns {
    fn run(&self, snap: &FileSnapshot, _ctx: &mut FindingContext<'_>) -> Vec<Advisory> {
        semantic_patterns::find(snap)
    }
}

/// Returns the registered finding modules in execution order.
pub fn registered_modules() -> Vec<Box<dyn FindingModule>> {
    vec![
        Box::new(DeadBranch),
        Box::new(UnusedVariable),
        Box::new(TemporalViolation),
        Box::new(HallucinatedImport),
        Box::new(CrossFileTaint),
        Box::new(AtomicSection),
        Box::new(SemanticPatterns),
    ]
}
