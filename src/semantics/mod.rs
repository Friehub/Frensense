// SPDX-License-Identifier: MIT

pub mod consistency;
pub mod data_flow;
pub mod graph;
pub mod reachability;
pub mod simple_taint;
pub mod symbols;

pub use data_flow::{DataFlowAnalyzer, TaintRegistry};
pub use graph::{EdgeKind, SemanticGraph};
pub use reachability::ReachabilityChecker;
pub use symbols::{Symbol, SymbolKind, SymbolRegistry};
