// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Program-graph construction: def-use made explicit and interprocedural.
//!
//! * [`svfg`], per-function sparse value-flow graph.
//! * [`callgraph`], dispatch resolution (aliases, methods, re-exports).
//! * [`heap`], points-to analysis (Andersen, inclusion-based).
//! * [`steensgaard`], unification points-to (near-linear; R4 phase 1).
//! * [`store`], serialize/reload the graph store (feature = "serialize").

pub mod callgraph;
pub mod heap;
pub mod steensgaard;
#[cfg(feature = "serialize")]
pub mod store;
pub mod svfg;

#[cfg(test)]
mod callgraph_tests;
#[cfg(test)]
mod heap_tests;
#[cfg(test)]
mod steensgaard_tests;
#[cfg(test)]
mod two_phase_tests;
