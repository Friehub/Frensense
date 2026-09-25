// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

use frensense_bundler::fact_extract::{extract_facts, group_families};
use frensense_engine::analysis::taint::config::TaintConfig;
use frensense_engine::analysis::taint::facts::{FactTable, LearnedFactEntry};
use std::path::Path;

fn config() -> TaintConfig {
    TaintConfig {
        sources: ["req.body", "req.query"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        sinks: ["query", "execute", "eval"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        sanitizers: ["escapeHtml"].iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn test_group_families_from_directory() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("fam_x_positive.ts"), "const a = 1;").unwrap();
    std::fs::write(dir.path().join("fam_x_negative.ts"), "const a = 2;").unwrap();
    std::fs::write(dir.path().join("fam_x_negative2.ts"), "const a = 3;").unwrap();
    std::fs::write(dir.path().join("unrelated.ts"), "const b = 4;").unwrap();
    let fams = group_families(dir.path()).unwrap();
    assert_eq!(fams.len(), 1);
    assert_eq!(fams[0].id, "fam_x");
    assert_eq!(fams[0].positives.len(), 1);
    assert_eq!(fams[0].negatives.len(), 2);
}

#[test]
fn test_extract_sanitizer_fact_from_healthy_pair() {
    // Positive: tainted value flows straight into a sink (alerts at baseline).
    // Negative: same flow but guarded by a `.test()` allowlist, engine
    // doesn't know `test` yet → negative alerts (FP). The family proposes
    // `test` as a guard sanitizer; the replay gate verifies the family
    // separates with it and the fact is published.
    let pos = r#"
import express from "express";
const router = express.Router();
router.post("/p", (req: any, res: any) => {
    const id = req.body.id;
    eval(id);
    res.json({});
});
export default router;
"#;
    let neg = r#"
import express from "express";
const router = express.Router();
const SAFE = /^[a-z0-9]+$/;
router.post("/p", (req: any, res: any) => {
    const id = req.body.id;
    if (SAFE.test(id)) {
        eval(id);
    }
    res.json({});
});
export default router;
"#;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("fam_guard_positive.ts"), pos).unwrap();
    std::fs::write(dir.path().join("fam_guard_negative.ts"), neg).unwrap();
    let fams = group_families(dir.path()).unwrap();

    let cfg = config();
    let builtin = FactTable::from_config(&cfg);

    // Baseline: positive alerts, but negative ALSO alerts (unknown sanitizer).
    let base = frensense_engine::scan::scan(&fams[0].positives, &cfg, &builtin);
    assert!(base.has_alert(), "positive must alert at baseline");
    let base_neg = frensense_engine::scan::scan(&fams[0].negatives, &cfg, &builtin);
    assert!(
        base_neg.has_alert(),
        "negative alerts at baseline (test unknown), this is the FP the fact should fix"
    );

    let (learned, published) = extract_facts(&fams, &cfg, &builtin);
    let has_test = published.iter().any(
        |f| matches!(&f.entry, LearnedFactEntry::Sanitizer { call, guard_style: true, .. } if call == "test"),
    );
    assert!(
        has_test,
        "expected a `test` guard fact; published: {:?}",
        published.iter().map(|f| &f.entry).collect::<Vec<_>>()
    );

    // The learned table must now separate the family.
    let after = frensense_engine::scan::scan(&fams[0].negatives, &cfg, &learned);
    assert!(
        !after.has_alert(),
        "negative must be clean with the learned fact; findings: {:?}",
        after.findings
    );
}
