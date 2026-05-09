// SPDX-License-Identifier: MIT

pub mod data_flow;
pub mod symbols;

pub use data_flow::{DataFlowAnalyzer, TaintRegistry};
pub use symbols::{Symbol, SymbolKind, SymbolRegistry};
