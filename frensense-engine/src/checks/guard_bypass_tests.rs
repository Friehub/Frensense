// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Tests for the guard-bypass / credential-policy seed checks.

#[cfg(test)]
pub mod guard_bypass_tests {
    use crate::checks::guard_bypass;
    use crate::harness::lower_source;

    /// The redirectChallenge shape: allowlist validation by substring
    /// containment over attacker-controlled URL input.
    #[test]
    fn substring_allowlist_guard_fires() {
        let src = r#"
export const isRedirectAllowed = (url: string) => {
  let allowed = false
  for (const allowedUrl of redirectAllowlist) {
    allowed = allowed || url.includes(allowedUrl)
  }
  return allowed
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        let hits: Vec<_> = fns.values().flat_map(guard_bypass::check).collect();
        assert!(!hits.is_empty(), "substring guard must fire");
        assert_eq!(hits[0].rule, "substring_allowlist_guard");
    }

    /// Non-URL containment (`list.includes(item)` on plain names) must NOT
    /// fire, the rule is scoped to URL/redirect-shaped validation.
    #[test]
    fn plain_includes_does_not_fire() {
        let src = r#"
export function hasItem (items: string[], needle: string) {
  return items.includes(needle)
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        let hits: Vec<_> = fns.values().flat_map(guard_bypass::check).collect();
        assert!(hits.is_empty(), "non-URL includes must stay silent");
    }

    /// The weakPasswordChallenge shape: a plaintext password parameter fed
    /// to a hash-named call.
    #[test]
    fn credential_kdf_fires() {
        let src = r#"
export function storePassword (clearTextPassword: string) {
  return security.hash(clearTextPassword)
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        let hits: Vec<_> = fns
            .values()
            .flat_map(guard_bypass::check_credentials)
            .collect();
        assert!(!hits.is_empty(), "credential KDF policy must fire");
        assert_eq!(hits[0].rule, "credential_kdf_policy");
    }

    /// Hashing a non-credential value must not fire.
    #[test]
    fn non_credential_hash_stays_silent() {
        let src = r#"
export function cacheKey (userId: string) {
  return crypto.hash(userId)
}
"#;
        let fns = lower_source("t.ts", src, "ts").unwrap();
        let hits: Vec<_> = fns
            .values()
            .flat_map(guard_bypass::check_credentials)
            .collect();
        assert!(hits.is_empty(), "non-credential hash must stay silent");
    }
}
