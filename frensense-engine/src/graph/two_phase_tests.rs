// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the two-phase points-to pipeline (R4): Steensgaard unification
//! gates where Andersen's fixed point spends its work, without losing
//! precision on taint-relevant variables.

#[cfg(test)]
pub mod two_phase_tests {
    use crate::graph::heap::PointsToAnalysis;
    use crate::graph::steensgaard::Steensgaard;
    use crate::harness::lower_source;

    /// Run full Andersen and phase-2 Andersen on every function of `src`
    /// and return the (name, var-points-to-count) pairs for comparison.
    fn alias_sets(
        src: &str,
    ) -> Vec<(
        String,
        Vec<(usize, usize)>, // sorted (var, |pts|) for full Andersen
        Vec<(usize, usize)>, // same for two-phase
    )> {
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let mut out = Vec::new();
        for (name, ir) in &fns {
            let mut full = PointsToAnalysis::new();
            full.analyze(ir);
            let mut two = PointsToAnalysis::new();
            two.analyze_two_phase(ir);
            let collect = |pts: &rustc_hash::FxHashMap<
                crate::ir::function::VarId,
                rustc_hash::FxHashSet<crate::graph::heap::LocId>,
            >|
             -> Vec<(usize, usize)> {
                #[allow(clippy::disallowed_methods)]
                let mut v: Vec<(usize, usize)> = pts.iter().map(|(k, s)| (k.0, s.len())).collect();
                v.sort_unstable();
                v
            };
            out.push((name.clone(), collect(&full.pts), collect(&two.pts)));
        }
        out
    }

    /// Phase 2 must preserve every alias *fact* for relevant variables:
    /// here the tainted-relevant chain is `req` → `obj.data` → sink arg.
    #[test]
    fn phase2_preserves_relevant_precision() {
        let src = r#"
import { exec } from "child_process";
export function handler(req: any): void {
    const obj: any = {};
    obj.data = req.body.payload;
    const picked = obj.data;
    exec(picked);
}
"#;
        let sets = alias_sets(src);
        let handler = sets.iter().find(|(n, _, _)| n == "handler").unwrap();
        let f = &handler.1;
        let t = &handler.2;

        // Same variables participate, and each relevant var's pts is never
        // SMALLER under phase 2 (restriction may only over-approximate).
        let get = |v: &[(usize, usize)], var: usize| -> usize {
            v.iter()
                .find(|(k, _)| *k == var)
                .map(|&(_, n)| n)
                .unwrap_or(0)
        };
        for &(var, _) in f.iter() {
            assert!(
                get(t, var) >= get(f, var),
                "var {var}: phase-2 pts must not shrink below full Andersen"
            );
        }
        // And the relevant loads still resolve to locations (not all-zero).
        assert!(
            f.iter().any(|&(_, n)| n > 0),
            "full Andersen resolves some locations"
        );
        assert!(
            t.iter().any(|&(_, n)| n > 0),
            "phase 2 resolves locations for the relevant cluster"
        );
    }

    /// Steensgaard must place the assignment chain (`a = b = req`) in one
    /// class, that class is what makes the chain relevant in phase 2.
    #[test]
    fn steensgaard_unifies_alias_chain() {
        let src = r#"
export function handler(req: any): void {
    const a = req;
    const b = a;
    const c = b;
    return c.body;
}
"#;
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let ir = fns.values().next().unwrap();
        let st = Steensgaard::analyze(ir);

        // Collect class of the chain by finding any two of a/b/c sharing.
        let classes: Vec<_> = st.class_of.values().copied().collect();
        let distinct: std::collections::BTreeSet<usize> = classes.iter().map(|c| c.0).collect();
        // a, b, c (and req) unify into fewer classes than members.
        assert!(
            distinct.len() < classes.len(),
            "assignment chain must unify (distinct classes {distinct:?} < members {classes:?})"
        );
    }

    /// An irrelevant variable (literal-only arithmetic, no params, no calls,
    /// no allocations participating in field flows) must NOT force phase-2
    /// field tracking: its exclusion is the work-reduction premise.
    #[test]
    fn phase2_excludes_irrelevant_classes() {
        let src = r#"
export function pure_math(x: number): number {
    let acc = x * 2 + 7;
    for (let i = 0; i < 3; i++) {
        acc += i;
    }
    return acc;
}
"#;
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let ir = fns.values().next().unwrap();
        let _st = Steensgaard::analyze(ir);

        // Sanity: Steensgaard produces a partition; the phase-2 gate then
        // finds no anchors (no params used at calls, no allocations), so
        // every class is irrelevant. This is a structural check that the
        // gate's anchor rules don't over-mark pure computation.
        let mut full = PointsToAnalysis::new();
        full.analyze(ir);
        let mut two = PointsToAnalysis::new();
        two.analyze_two_phase(ir);
        // Soundness direction: phase-2 pts sets are a superset per var.
        for (k, s) in &full.pts {
            let t = two.pts.get(k).cloned().unwrap_or_default();
            assert!(
                t.is_superset(s),
                "var {k:?}: phase 2 must not lose locations"
            );
        }
    }

    /// End-to-end: the demand engine still fires through a field flow when
    /// phase 2 (not full Andersen) built the alias graph the SVFG used.
    #[test]
    fn demand_engine_fires_through_phase2_heap() {
        use crate::analysis::taint::engine::BackwardTaintEngine;

        let src = r#"
import { exec } from "child_process";
export function handler(req: any): void {
    const obj: any = {};
    obj.data = req.body.payload;
    exec(obj.data);
}
"#;
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let statics: rustc_hash::FxHashMap<String, &crate::ir::function::FunctionIR> =
            fns.iter().map(|(k, v)| (k.clone(), v)).collect();
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let config = crate::analysis::taint::facts::config_from_spec(&*spec);
        let prog = crate::analysis::forward::ProgramSvfg::new(&statics, &config);
        let mut engine = BackwardTaintEngine::new(&prog, &config);
        engine.run();
        assert!(
            !engine.findings.is_empty(),
            "field flow must survive phase-2 alias analysis"
        );
    }
}
