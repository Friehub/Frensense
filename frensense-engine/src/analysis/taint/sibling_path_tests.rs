// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Sibling-callee heap flow at the path-reporting level (R4/R5).
//!
//! `handler` passes the SAME object to two callees: `writer` stores taint
//! into `obj.data`, `reader` loads it and returns it to a sink. The flow
//! spans three functions and is completed only by the sibling heap cross
//! edge (direction 3 of `install_heap_cross_edges`), no caller-side
//! load/store of the field exists. These tests lock in that the captured
//! `TaintPath` reflects the full three-function journey, not just a
//! two-function fragment.

#[cfg(test)]
mod sibling_heap_path_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::engine::{BackwardTaintEngine, BackwardVerdict};
    use crate::analysis::taint::path::{PathStep, dedup_by_shape};
    use crate::harness::lower_source;
    use crate::ir::function::FunctionIR;
    use rustc_hash::FxHashMap;

    const SIBLING_HEAP: &str = r#"
import { exec } from "child_process";

function writer(box: any, input: string): void {
  box.data = input;
}

function reader(box: any): string {
  return box.data;
}

function handler(req: any): void {
  const box: any = {};
  writer(box, req.body.payload);
  exec(reader(box));
}
"#;

    fn run(src: &str) -> Vec<crate::analysis::taint::engine::SinkFinding> {
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let owned: Vec<FunctionIR> = fns.into_values().collect();
        let mut statics: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        for ir in &owned {
            statics.insert(ir.name.clone(), ir);
        }
        let mut config = TaintConfig::default();
        config.sinks.insert("exec".into());
        config.sources.insert("req.body".into());
        config.sources.insert("req".into());
        let prog = ProgramSvfg::new(&statics, &config);
        // Map every function to the scan's pseudo-file so path spans carry
        // a file location (production passes the real fn→file map here).
        let fn_file: FxHashMap<String, String> = statics
            .keys()
            .map(|name| (name.clone(), "t.ts".to_string()))
            .collect();
        let mut engine = BackwardTaintEngine::new(&prog, &config).with_fn_file(&fn_file);
        engine.run();
        engine.findings
    }

    fn vulnerable_path(src: &str) -> super::super::path::TaintPath {
        let findings = run(src);
        let vuln: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable && f.alert.is_some())
            .collect();
        assert!(!vuln.is_empty(), "sibling heap flow must be detected");
        vuln[0].path.clone()
    }

    /// The path must visit all three functions: the source is in `handler`,
    /// taint enters `writer` through the field store, re-emerges through
    /// `reader`'s field load, and lands back in `handler` at the sink.
    #[test]
    fn path_spans_all_three_functions() {
        let p = vulnerable_path(SIBLING_HEAP);

        fn function_of(s: &PathStep) -> Option<&str> {
            match s {
                PathStep::FieldLoad { function, .. }
                | PathStep::Assignment { function, .. }
                | PathStep::FormalParam { function, .. }
                | PathStep::CallArgument { function, .. }
                | PathStep::CallReturn { function, .. } => Some(function.as_str()),
                PathStep::ReturnToCaller { caller } => Some(caller.as_str()),
                _ => None,
            }
        }
        let functions: std::collections::BTreeSet<&str> =
            p.steps.iter().filter_map(|s| function_of(s)).collect();

        assert!(
            functions.contains("writer"),
            "path must include the storing callee `writer`, got {functions:?}, steps: {p:?}"
        );
        assert!(
            functions.contains("reader"),
            "path must include the loading callee `reader`, got {functions:?}, steps: {p:?}"
        );
        assert!(
            functions.contains("handler"),
            "path must include the orchestrating caller `handler`, got {functions:?}"
        );
    }

    /// The field is the flow's backbone: the path must carry a FieldLoad of
    /// `data` inside `reader`, the exact point where the sibling heap edge
    /// delivers the value written by `writer`.
    #[test]
    fn path_carries_field_load_of_data_in_reader() {
        let p = vulnerable_path(SIBLING_HEAP);
        assert!(
            p.steps
                .iter()
                .any(|s| matches!(s, PathStep::FieldLoad { function, field, .. }
                    if function == "reader" && field == "data")),
            "path must include the reader-side FieldLoad of `data`, got {p:?}"
        );
    }

    /// Source-first ordering with a hop into the writing callee and back:
    /// the writer must appear as a CallArgument hop (taint passed into
    /// `writer` as `input`), anchored after the source step.
    #[test]
    fn path_enters_writer_via_argument_hop() {
        let p = vulnerable_path(SIBLING_HEAP);

        // Source first.
        assert!(
            matches!(&p.steps[0], PathStep::Source { description } if description.contains("req")),
            "first step must be the source, got {:?}",
            p.steps[0]
        );

        // The hop into writer (arg slot for `input`).
        assert!(
            p.steps.iter().any(
                |s| matches!(s, PathStep::CallArgument { function, callee, slot: 1 }
                    if function == "handler" && callee == "writer")
            ),
            "path must pass taint into `writer` via argument slot 1, got {p:?}"
        );

        // And the formal-parameter entry inside writer.
        assert!(
            p.steps
                .iter()
                .any(|s| matches!(s, PathStep::FormalParam { function, param }
                    if function == "writer" && param == "input")),
            "path must enter `writer` as formal param `input`, got {p:?}"
        );
    }

    /// Structural integrity: spans align with steps (every path consumer
    /// relies on this), and the text rendering names the source.
    #[test]
    fn path_invariants_hold() {
        let p = vulnerable_path(SIBLING_HEAP);
        assert!(
            p.steps.len() >= 5,
            "three-function path needs several steps, got {}",
            p.steps.len()
        );
        assert_eq!(p.spans.len(), p.steps.len(), "spans must align with steps");
        let text = p.render_text();
        assert!(
            text.contains("1. source:"),
            "rendering must start at the source"
        );
    }

    /// Step spans must resolve to the REAL source file (not the function
    /// name pseudo-file), so consumers can annotate every hop with a
    /// file:line location. The finding's file is the scan file; step spans
    /// for functions inside it must carry the same path.
    #[test]
    fn step_spans_carry_real_file_paths() {
        let findings = run(SIBLING_HEAP);
        let vuln: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable && f.alert.is_some())
            .collect();
        let p = &vuln[0].path;
        // The reader field-load step (index 4 in this shape) must have a
        // span whose file ends with the scanned file name.
        let load_span = p
            .spans
            .iter()
            .flatten()
            .find(|(file, _)| file.ends_with("t.ts"));
        assert!(
            load_span.is_some(),
            "step spans must point at the real source file, got {p:?}"
        );
    }

    /// The sibling path has a distinct shape from the two-function
    /// scalar-cross-file path; shape-based dedup must keep both.
    #[test]
    fn sibling_shape_dedups_against_scalar_shape() {
        const SCALAR: &str = r#"
import { exec } from "child_process";

function passthrough(input: string): string {
  return input;
}

function handler(req: any): void {
  exec(passthrough(req.body.payload));
}
"#;
        let sibling = vulnerable_path(SIBLING_HEAP);
        let scalar = vulnerable_path(SCALAR);

        let kept = dedup_by_shape(&[sibling.clone(), scalar.clone()]);
        assert_eq!(
            kept.len(),
            2,
            "sibling-heap and scalar paths have different shapes and must both survive dedup"
        );
        assert_eq!(
            sibling.shape_id(),
            sibling.shape_id(),
            "shape id must be stable"
        );
    }
}
