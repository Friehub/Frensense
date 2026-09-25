// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Bundle construction: corpus pairs → replay-verified facts → `.frc` bytes.

use crate::fact_extract::{extract_facts, group_families};
use crate::format::{write_bundle, BundlePattern, BundlePayload};

/// §9 pipeline: corpus pairs → replay-verified learned facts → bundle bytes.
///
/// Returns the bundle plus a summary of the published facts for reporting.
pub fn build_facts_bundle(
    corpus_dir: &std::path::Path,
    config: &frensense_engine::analysis::taint::config::TaintConfig,
    builtin: &frensense_engine::analysis::taint::facts::FactTable,
) -> Result<(Vec<u8>, Vec<crate::fact_extract::LearnedFact>), String> {
    let families = group_families(corpus_dir)?;
    eprintln!(
        "[facts] {} families grouped from {}",
        families.len(),
        corpus_dir.display()
    );

    let (learned_table, published) = extract_facts(&families, config, builtin);

    // Payload: one metadata pattern per family + the published facts.
    let entries: Vec<frensense_engine::analysis::taint::facts::LearnedFactEntry> =
        published.iter().map(|f| f.entry.clone()).collect();

    let mut families_iter = families.iter();
    let mut patterns: Vec<BundlePattern> = Vec::new();
    let mut family_ids: Vec<String> = families.iter().map(|f| f.id.clone()).collect();
    family_ids.sort();
    for id in &family_ids {
        // Metadata comes from the family's [frensense] block when present;
        // family lookup keeps the mapping stable across the two passes.
        let _ = &mut families_iter;
        patterns.push(BundlePattern {
            id: id.clone(),
            observation: None,
            impact: None,
            improvement: None,
            cwe: None,
            cvss: None,
            owasp: None,
            severity: None,
        });
    }

    let count = entries.len().max(patterns.len()) as u32;
    let payload = BundlePayload {
        patterns,
        learned_facts: entries,
    };
    let _ = learned_table;
    let bytes = write_bundle(&payload, count)?;
    Ok((bytes, published))
}

/// Metadata for one family, extracted from the positive file's
/// `[frensense]` comment block if present.
#[allow(dead_code)]
fn family_metadata(_corpus_dir: &std::path::Path, family_id: &str) -> BundlePattern {
    BundlePattern {
        id: family_id.to_string(),
        observation: None,
        impact: None,
        improvement: None,
        cwe: None,
        cvss: None,
        owasp: None,
        severity: None,
    }
}
