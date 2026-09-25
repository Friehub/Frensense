// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Bundle payload types.
//!
//! A `.frc` bundle is two things: per-family advisory metadata
//! ([`BundlePattern`]) and the machine-consumable knowledge
//! (`learned_facts`, replay-verified sink/sanitizer facts the engine
//! merges into its fact table).

use frensense_engine::analysis::taint::facts::LearnedFactEntry;

use super::frc::{read_bundle, BundleHeader};

/// Per-family advisory metadata.
///
/// The facts pipeline (`--facts`) writes one pattern per corpus family
/// carrying the human-facing advisory fields (CWE, CVSS, OWASP, severity).
/// Flow knowledge is NOT here, it is in `learned_facts`, which the engine
/// consumes.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BundlePattern {
    pub id: String,
    #[serde(default)]
    pub observation: Option<String>,
    #[serde(default)]
    pub impact: Option<String>,
    #[serde(default)]
    pub improvement: Option<String>,
    #[serde(default)]
    pub cwe: Option<String>,
    #[serde(default)]
    pub cvss: Option<f32>,
    #[serde(default)]
    pub owasp: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Bundle {
    pub header: BundleHeader,
    pub patterns: Vec<BundlePattern>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BundlePayload {
    pub patterns: Vec<BundlePattern>,
    /// Learned facts (§9): sink signatures + sanitizer facts extracted from
    /// positive/negative pairs, replay-verified at bundle build time.
    #[serde(default)]
    pub learned_facts: Vec<LearnedFactEntry>,
}

pub struct LoadedBundle {
    pub patterns: Vec<BundlePattern>,
    /// Learned facts from §9 (empty for legacy bundles).
    pub learned_facts: Vec<LearnedFactEntry>,
}

pub fn load_bundle(bytes: &[u8]) -> Result<LoadedBundle, String> {
    let (header, patterns, learned_facts) = match read_bundle::<BundlePayload>(bytes) {
        Ok((h, payload)) => (h, payload.patterns, payload.learned_facts),
        Err(e) => match read_bundle::<Vec<LegacyPattern>>(bytes) {
            Ok((h, _)) => {
                tracing::warn!(
                    "legacy fingerprint bundle loaded (version {}); no facts available",
                    h.version
                );
                (h, Vec::new(), Vec::new())
            }
            Err(_) => return Err(format!("Failed to deserialize bundle: {}", e)),
        },
    };

    if patterns.len() < header.pattern_count as usize {
        tracing::warn!(
            "bundle pattern count mismatch (expected {}, loaded {})",
            header.pattern_count,
            patterns.len()
        );
    }

    Ok(LoadedBundle {
        patterns,
        learned_facts,
    })
}

/// Legacy bundles serialized a bare pattern vector with fingerprint
/// payloads. We accept the envelope but discard the shape data.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LegacyPattern {
    #[serde(default)]
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_version_check() {
        let bytes = vec![b'F', b'R', b'C', b'1'];
        assert!(load_bundle(&bytes).is_err());
    }
}
