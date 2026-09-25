// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the schema-policy and allowlist-definition seed checks (R1),
//! including the multi-line span reporting that lets consumers match a
//! finding against any line its span covers.

#[cfg(test)]
pub mod schema_policy_tests {
    use crate::analysis::taint::facts::FactTable;
    use crate::checks::{check_all, schema_policy};
    use crate::harness::lower_source;

    /// The chatbot tool-schema shape: a z.number() whose describe text
    /// declares a maximum in prose, with no .max() enforcement anywhere.
    const UNBOUNDED: &str = r#"
import { tool } from 'ai'
import { z } from 'zod'
export const tools = {
  generateCoupon: tool({
    description: 'Generate a discount coupon (maximum 10)',
    inputSchema: z.object({
      discount: z.number().describe('The discount percentage (maximum 10)')
    }),
    execute: async ({ discount }) => {
      return { coupon: discount }
    }
  })
}
"#;

    fn run(src: &str) -> Vec<crate::checks::CheckerFinding> {
        let irs = lower_source("test.ts", src, "ts").expect("lower");
        check_all(irs.values(), &FactTable::default())
    }

    #[test]
    fn unbounded_number_schema_fires_on_prose_bound() {
        let findings = run(UNBOUNDED);
        let f = findings
            .iter()
            .find(|f| f.rule == "unbounded_number_schema")
            .expect("unbounded schema must fire");
        assert!(
            f.message.contains("maximum 10"),
            "message quotes the declared policy"
        );
        assert!(f.span.is_some(), "span recorded for line reporting");
    }

    #[test]
    fn bounded_schema_is_silent() {
        // The .max() enforcer applies the bound the prose declares: no finding.
        let src = r#"
import { z } from 'zod'
export const schema = {
  discount: z.number().max(10).describe('maximum 10 percent')
}
"#;
        assert!(
            run(src).iter().all(|f| f.rule != "unbounded_number_schema"),
            "enforced schema must not fire"
        );
    }

    #[test]
    fn prose_without_numbers_is_silent() {
        // Describes semantics, not a numeric policy, no bound claim.
        let src = r#"
import { z } from 'zod'
export const schema = {
  name: z.string().describe('The customer name')
}
"#;
        assert!(
            run(src).iter().all(|f| f.rule != "unbounded_number_schema"),
            "no numeric prose, no finding"
        );
    }

    #[test]
    fn allowlist_definition_fires_with_containment_guard() {
        // The redirectChallenge shape: URL Set + includes() guard elsewhere.
        let src = r#"
export const redirectAllowlist = new Set([
  'https://github.com/juice-shop/juice-shop',
  'https://blockchain.info/address/1AbKfgvw9psQ41NbLi8kufDQTezwG8DRZm'
])
export const isRedirectAllowed = (url: string) => {
  let allowed = false
  for (const allowedUrl of redirectAllowlist) {
    allowed = allowed || url.includes(allowedUrl)
  }
  return allowed
}
"#;
        let findings = run(src);
        let def = findings
            .iter()
            .find(|f| f.rule == "allowlist_definition_bypassable")
            .expect("allowlist definition must fire alongside the guard");
        let (start, end) = def.span.expect("definition span");
        assert!(end > start, "definition span is multi-line");
        // The guard itself fires too.
        assert!(
            findings
                .iter()
                .any(|f| f.rule == "substring_allowlist_guard"),
            "guard rule fires"
        );
    }

    #[test]
    fn allowlist_without_containment_guard_is_silent() {
        // Exact-match guard: the Set is fine, the definition rule must not fire.
        let src = r#"
export const redirectAllowlist = new Set([
  'https://github.com/juice-shop/juice-shop'
])
export const isRedirectAllowed = (url: string) => redirectAllowlist.has(url)
"#;
        assert!(
            run(src)
                .iter()
                .all(|f| f.rule != "allowlist_definition_bypassable"),
            "no containment guard, no definition finding"
        );
    }
}
