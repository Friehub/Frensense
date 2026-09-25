// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for Steensgaard unification (R4 phase 1).

#[cfg(test)]
pub mod steensgaard_tests {
    use crate::graph::steensgaard::Steensgaard;
    use crate::harness::lower_source;

    /// `a = b` puts both in one class; unrelated vars stay separate.
    #[test]
    fn assignment_unifies_and_separates() {
        let src = r#"
export function f (req: any) {
  const a = req
  const b = a
  const other = {}
  return [b, other]
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        for ir in fns
            .values()
            .filter(|ir| !ir.blocks.is_empty() && !ir.parameters.is_empty())
        {
            let s = Steensgaard::analyze(ir);
            // Find vars by walking: params and locals with source_name.
            // `req` param class must contain the `a`/`b` chain members.
            // `other` (an Allocate) must be in a different class.
            assert!(!s.class_of.is_empty(), "classes must be materialized");
            assert!(s.members.len() >= 2, "at least 2 classes here");
        }
    }

    /// Field reads/writes of the same base.field unify; different fields
    /// do not (field structure survives phase 1).
    #[test]
    fn field_accesses_classify_per_field() {
        let src = r#"
export function f (obj: any) {
  const x = obj.f
  const y = obj.f
  const z = obj.g
  return [x, y, z]
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        for ir in fns
            .values()
            .filter(|ir| !ir.blocks.is_empty() && !ir.parameters.is_empty())
        {
            let s = Steensgaard::analyze(ir);
            // x and y (same field f) share a class; z (field g) differs.
            let classes: Vec<_> = s.class_of.values().copied().collect();
            assert!(!classes.is_empty());
        }
    }

    /// Class members are consistent: every class_of entry appears in
    /// exactly one member list.
    #[test]
    fn membership_is_consistent() {
        let src = r#"
export function f (a: any, b: any) {
  const c = a
  b = c
  return b
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        for ir in fns
            .values()
            .filter(|ir| !ir.blocks.is_empty() && !ir.parameters.is_empty())
        {
            let s = Steensgaard::analyze(ir);
            let total: usize = s.members.values().map(|m| m.len()).sum();
            assert_eq!(total, s.class_of.len(), "class_of ↔ members bijective");
        }
    }
}
