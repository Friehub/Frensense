// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the session-trust sanitizer: values derived from a trusted
//! session store's accessor (`authenticatedUsers.get(token)`) are
//! server-issued, not attacker-controlled, so taint through the store
//! boundary is severed. Receiver-aware: `myMap.get(t)` stays tainted.

#[cfg(test)]
pub mod session_trust_tests {
    use crate::analysis::taint::engine::BackwardVerdict;
    use crate::analysis::taint::facts::{config_from_spec, fact_table_from_spec};
    use crate::harness::lower_source;
    use crate::scan::{prepare, scan_prepared};

    /// The Juice Shop session pattern: token -> session store -> identity
    /// field -> finder sink. The identity field is server-issued; the flow
    /// must be severed at the store boundary.
    const SESSION_FLOW: &str = r#"
import { Request, Response } from 'express'
import * as security from './security'
declare const ordersCollection: any
export function orderHistory () {
  return async (req: Request, res: Response) => {
    const loggedInUser = security.authenticatedUsers.get(req.headers?.authorization?.replace('Bearer ', ''))
    if (loggedInUser?.data?.email && loggedInUser.data.id) {
      const email = loggedInUser.data.email
      const order = await ordersCollection.find({ email: email })
      res.status(200).json({ data: order })
    }
  }
}
"#;

    /// A plain Map.get with tainted args is NOT a session accessor.
    const MAP_FLOW: &str = r#"
declare const myMap: any
export function handler (req: any) {
  const v = myMap.get(req.body.key)
  return { value: v }
}
"#;

    fn run(src: &str) -> Vec<crate::analysis::taint::engine::SinkFinding> {
        let files = vec![("test.ts".to_string(), src.to_string(), "ts".to_string())];
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let config = config_from_spec(spec);
        let mut facts = fact_table_from_spec(spec);
        // Session roots are corpus-owned seed facts now, not spec tables,
        // apply the same seed document production loads.
        crate::analysis::taint::facts::seed::SeedFacts::parse(
            r#"{"session_roots": ["authenticatedUsers"]}"#,
        )
        .expect("seed")
        .apply_to(&mut facts);
        let prepared = prepare(&files).expect("prepare");
        scan_prepared(&prepared, &config, &facts).findings
    }

    #[test]
    fn session_derived_identity_is_not_a_flow() {
        let findings = run(SESSION_FLOW);
        let find_f = findings
            .iter()
            .find(|f| f.sink == "find" && f.verdict == BackwardVerdict::Vulnerable);
        assert!(
            find_f.is_none(),
            "session-derived identity must not alert: {:?}",
            find_f.map(|f| (&f.sink, &f.source_desc))
        );
    }

    #[test]
    fn plain_map_get_stays_a_flow() {
        // No sink in MAP_FLOW; the point is the engine's session predicate
        // doesn't fire on arbitrary receivers, assert via the fact table
        // directly for precision.
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let mut facts = fact_table_from_spec(spec);
        crate::analysis::taint::facts::seed::SeedFacts::parse(
            r#"{"session_roots": ["authenticatedUsers"]}"#,
        )
        .expect("seed")
        .apply_to(&mut facts);
        assert!(!facts.is_session_path("get", Some("myMap")));
        assert!(facts.is_session_path("get", Some("security.authenticatedUsers")));
        assert!(!facts.is_session_path("put", Some("security.authenticatedUsers")));
    }

    #[test]
    fn receiver_access_path_resolves_namespaced_stores() {
        let irs = lower_source("t.ts", SESSION_FLOW, "ts").unwrap();
        let (_, ir) = irs
            .iter()
            .find(|(n, _)| n.contains("orderHistory") || n.contains("handler"))
            .expect("fn");
        // Some var in the function must resolve to a path containing the
        // session root.
        let any_path = ir.var_metadata.keys().any(|v| {
            crate::analysis::taint::facts::FactTable::receiver_access_path(ir, *v)
                .map(|p| p.contains("authenticatedUsers"))
                .unwrap_or(false)
        });
        assert!(
            any_path,
            "receiver_access_path must reach the store segment"
        );
    }
}
