// SPDX-License-Identifier: MIT

// The taint module previously contained TaintTracker and TaintLookup,
// which were early prototypes superseded by:
//   - engine/src/data_flow/engine.rs    (DataFlowEngine — summaries, global taint)
//   - engine/src/data_flow/resolver.rs  (resolve_fn_definition, map_call_args_to_params)
//   - src/semantics/data_flow/          (DataFlowAnalyzer — full intra/inter-procedural)
//
// The TaintRegistry itself lives in data_flow/mod.rs and is the shared
// state container used across all taint analysis paths.
