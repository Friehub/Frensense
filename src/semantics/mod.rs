// [LICENSE] Proprietary - Friehub (GenSense Gateway)
// Copyright (c) 2026 Friehub. All rights reserved.

pub mod data_flow;
pub mod symbols;

pub use data_flow::{DataFlowAnalyzer, TaintRegistry};
pub use symbols::{Symbol, SymbolKind, SymbolRegistry};
