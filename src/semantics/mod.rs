// SPDX-License-Identifier: MIT

pub mod consistency;
pub mod data_flow;
pub mod simple_taint;

pub use data_flow::{DataFlowAnalyzer, TaintRegistry};

pub use frensense_engine::graph;
pub use frensense_engine::graph::{EdgeKind, SemanticGraph};
pub use frensense_engine::symbols;
pub use frensense_engine::symbols::{Symbol, SymbolKind, SymbolRegistry};
