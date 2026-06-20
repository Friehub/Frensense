use crate::Advisory;
use frensense_engine::data_flow::DataFlowEngine;

pub fn find(
    _symbols: &crate::semantics::symbols::SymbolRegistry,
    _snap: &crate::engine::project::FileSnapshot,
    _data_flow: Option<&DataFlowEngine>,
) -> Vec<Advisory> {
    // Cross-file taint detection is now handled by the corpus layer.
    // This module is retained for future corpus-based interprocedural analysis.
    Vec::new()
}
