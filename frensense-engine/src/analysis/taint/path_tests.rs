// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for taint-path capture (R3c).

#[cfg(test)]
mod path_capture_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::engine::{BackwardTaintEngine, BackwardVerdict};
    use crate::analysis::taint::path::{PathStep, dedup_by_shape};
    use crate::harness::lower_source;
    use crate::ir::function::FunctionIR;
    use rustc_hash::FxHashMap;

    const CROSS_FILE: &str = r#"
import { Pool } from "pg";
const pool = new Pool();

function buildQuery(id: string): string {
  return "SELECT * FROM users WHERE id = '" + id + "'";
}

function handler(req: any): void {
  const q = buildQuery(req.body.id);
  pool.query(q);
}
"#;

    fn run(src: &str) -> Vec<super::super::engine::SinkFinding> {
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let owned: Vec<FunctionIR> = fns.into_values().collect();
        let mut statics: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        for ir in &owned {
            statics.insert(ir.name.clone(), ir);
        }
        let mut config = TaintConfig::default();
        config.sinks.insert("query".into());
        config.sources.insert("req.body".into());
        config.sources.insert("req".into());
        let prog = ProgramSvfg::new(&statics, &config);
        let mut engine = BackwardTaintEngine::new(&prog, &config);
        engine.run();
        engine.findings
    }

    #[test]
    fn interprocedural_path_source_to_sink() {
        let findings = run(CROSS_FILE);
        let vuln: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable)
            .collect();
        assert!(!vuln.is_empty(), "probe must produce a vulnerable finding");

        let p = &vuln[0].path;
        // First step describes the source.
        assert!(
            matches!(&p.steps[0], PathStep::Source { description } if description.contains("req.body")),
            "first step must be the source, got {:?}",
            p.steps[0]
        );
        // The path crosses into buildQuery and returns.
        assert!(
            p.steps.iter().any(
                |s| matches!(s, PathStep::CallReturn { callee, .. } if callee == "buildQuery")
            ),
            "path must include the call-return hop, got {p:?}"
        );
        assert!(
            p.steps.iter().any(
                |s| matches!(s, PathStep::FormalParam { function, .. } if function == "buildQuery")
            ),
            "path must include entering the callee as a parameter"
        );
        // Source-first ordering.
        assert!(p.steps.len() >= 3, "path should have multiple steps");
        assert_eq!(p.spans.len(), p.steps.len(), "spans must align with steps");
    }

    #[test]
    fn non_vulnerable_findings_have_empty_path() {
        let findings = run(CROSS_FILE);
        for f in &findings {
            if f.verdict != BackwardVerdict::Vulnerable {
                assert!(
                    f.path.steps.is_empty(),
                    "non-vulnerable findings must not carry a path"
                );
            }
        }
    }

    #[test]
    fn same_shape_dedups_to_one() {
        let findings = run(CROSS_FILE);
        let paths: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable)
            .map(|f| f.path.clone())
            .collect();
        // Identical shape paths (same flow reported once) dedup to one.
        let kept = dedup_by_shape(&paths);
        assert!(!kept.is_empty());
        // Deduping a single path returns itself.
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn shape_id_ignores_variable_numbering() {
        let a = super::super::path::TaintPath {
            steps: vec![
                PathStep::Source {
                    description: "req.body".into(),
                },
                PathStep::Assignment {
                    function: "f".into(),
                    variable: 7,
                },
            ],
            spans: vec![None, None],
        };
        let b = super::super::path::TaintPath {
            steps: vec![
                PathStep::Source {
                    description: "req.body".into(),
                },
                PathStep::Assignment {
                    function: "f".into(),
                    variable: 99,
                },
            ],
            spans: vec![None, None],
        };
        assert_eq!(a.shape_id(), b.shape_id());
    }

    #[test]
    fn render_text_is_numbered_source_first() {
        let p = super::super::path::TaintPath {
            steps: vec![
                PathStep::Source {
                    description: "req.body".into(),
                },
                PathStep::Assignment {
                    function: "f".into(),
                    variable: 1,
                },
            ],
            spans: vec![None, None],
        };
        let text = p.render_text();
        assert!(text.contains("1. source:"));
        assert!(text.contains("2. `f`"));
    }
}

#[cfg(test)]
mod idor_classification_tests {
    use crate::analysis::forward::ProgramSvfg;
    use crate::analysis::taint::config::TaintConfig;
    use crate::analysis::taint::engine::{BackwardTaintEngine, BackwardVerdict, FindingClass};
    use crate::harness::lower_source;
    use crate::ir::function::FunctionIR;
    use rustc_hash::FxHashMap;

    fn run(src: &str) -> Vec<crate::analysis::taint::engine::SinkFinding> {
        let fns = lower_source("t.ts", src, "ts").expect("lower");
        let owned: Vec<FunctionIR> = fns.into_values().collect();
        let mut statics: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        for ir in &owned {
            statics.insert(ir.name.clone(), ir);
        }
        let mut config = TaintConfig::default();
        config.sinks.insert("findOne".into());
        config.sinks.insert("query".into());
        config.sources.insert("req.body".into());
        config.sources.insert("req".into());
        let prog = ProgramSvfg::new(&statics, &config);
        let spec = frensense_lang::spec_for_ext("ts").expect("ts spec");
        let mut engine = BackwardTaintEngine::new(&prog, &config)
            .with_fact_table(&crate::analysis::taint::facts::fact_table_from_spec(spec));
        engine.run();
        engine.findings
    }

    /// `findOne({ where: { id: req.body.x } })`, object-payload shape → Idor.
    #[test]
    fn object_payload_is_idor_class() {
        let src = r#"
export function updateProfile(req: any, db: any): void {
  db.User.findOne({ where: { id: req.body.userId } });
}
"#;
        let findings = run(src);
        let vuln: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable && f.alert.is_some())
            .collect();
        assert!(!vuln.is_empty(), "flow must be detected");
        assert!(
            vuln.iter().any(|f| f.finding_class == FindingClass::Idor),
            "object-payload flow must classify as Idor, got {:?}",
            vuln.iter().map(|f| f.finding_class).collect::<Vec<_>>()
        );
    }

    /// `query("SELECT..." + req.body.x)`, raw string concat → Injection.
    #[test]
    fn raw_string_is_injection_class() {
        let src = r#"
export function getUser(req: any, pool: any): void {
  pool.query("SELECT * FROM users WHERE id = '" + req.body.id + "'");
}
"#;
        let findings = run(src);
        let vuln: Vec<_> = findings
            .iter()
            .filter(|f| f.verdict == BackwardVerdict::Vulnerable && f.alert.is_some())
            .collect();
        assert!(!vuln.is_empty(), "flow must be detected");
        assert!(
            vuln.iter()
                .all(|f| f.finding_class == FindingClass::Injection),
            "raw string flow must classify as Injection, got {:?}",
            vuln.iter().map(|f| f.finding_class).collect::<Vec<_>>()
        );
    }
}
