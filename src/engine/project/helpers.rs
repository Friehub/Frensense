// SPDX-License-Identifier: MIT

#[cfg(feature = "fingerprinting")]
use super::super::fingerprint::FunctionFingerprint;
use super::Engine;
use crate::{Advisory, FileId, SourceRegistry};
use std::path::Path;

impl Engine {
    #[must_use]
    pub fn run_governance_checks(&self, root: &Path) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let sbom_txt = root.join("sbom.txt");
        let bom_json = root.join("bom.json");
        if !sbom_txt.exists() && !bom_json.exists() {
            advisories.push(Advisory {
                rule_id: "MISSING_SBOM".to_string(),
                file_id: FileId(0),
                file_path: root.to_string_lossy().to_string(),
                severity: crate::Severity::Warning,
                observation: "Project Health: No Software Bill of Materials (SBOM) found.".to_string(),
                impact: "Supply Chain Security: A verifiable SBOM is recommended for production-grade systems to track dependencies.".to_string(),
                improvement: "Generate an SBOM using 'cargo cyclonedx' and place it at 'bom.json'.".to_string(),
                line: 0,
                column: 0,
                start_byte: 0,
                end_byte: 0,
                original_content: "sbom.txt / bom.json".to_string(),
                proposed_replacement: None,
                proposed_import: None,
                enclosing_symbol: None,
                confidence: 1.0,
                fingerprint: String::new(),
                auto_fixable: false,
                requires_human: true,
                tags: vec![],
            });
        }
        advisories
    }

    #[cfg(feature = "fingerprinting")]
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn post_process_ngrams(
        &self,
        fingerprints: &[FunctionFingerprint],
        sources: &SourceRegistry,
    ) -> Vec<Advisory> {
        if fingerprints.len() < 2 {
            return Vec::new();
        }

        // Precompute n-gram set sizes for fast early-exit checks
        let ngram_sizes: Vec<usize> = fingerprints
            .iter()
            .map(|fp| fp.ngram_hashes.len())
            .collect();

        // Group by file path so we compare within files first
        let mut by_file: std::collections::HashMap<&str, Vec<usize>> =
            std::collections::HashMap::new();
        for (i, fp) in fingerprints.iter().enumerate() {
            by_file.entry(&fp.file_path).or_default().push(i);
        }

        // Build shared-hash index (hash → list of function indices that have it)
        let mut similarity_map: std::collections::HashMap<u64, Vec<usize>> =
            std::collections::HashMap::new();
        for (idx, fp) in fingerprints.iter().enumerate() {
            for &hash in &fp.ngram_hashes {
                similarity_map.entry(hash).or_default().push(idx);
            }
        }

        // Determine comparison order: within-file pairs first (more likely to match)
        let mut pairs: Vec<(usize, usize)> = Vec::new();

        // 1. Within-file comparisons
        for indices in by_file.values() {
            for a in 0..indices.len() {
                for b in (a + 1)..indices.len() {
                    pairs.push((indices[a], indices[b]));
                }
            }
        }

        // 2. Cross-file comparisons (only for functions sharing at least one n-gram)
        let file_of: Vec<&str> = fingerprints
            .iter()
            .map(|fp| fp.file_path.as_str())
            .collect();
        for (i, f1) in fingerprints.iter().enumerate() {
            let mut candidates = std::collections::HashSet::new();
            for &hash in &f1.ngram_hashes {
                if let Some(indices) = similarity_map.get(&hash) {
                    for &j in indices {
                        if j > i && file_of[i] != file_of[j] {
                            candidates.insert(j);
                        }
                    }
                }
            }
            for &j in &candidates {
                pairs.push((i, j));
            }
        }

        let threshold = 0.8;
        let mut advisories = Vec::new();
        let mut compared = std::collections::HashSet::new();

        for &(i, j) in &pairs {
            if !compared.insert((i, j)) {
                continue;
            }

            let f1 = &fingerprints[i];
            let f2 = &fingerprints[j];

            // Skip functions with trivial n-gram sets
            if ngram_sizes[i] < 3 || ngram_sizes[j] < 3 {
                continue;
            }

            // Early exit: Jaccard similarity is bounded by min/max set size ratio.
            // If the ratio is below the threshold, full computation can't reach it.
            let (small, large) = if ngram_sizes[i] < ngram_sizes[j] {
                (ngram_sizes[i], ngram_sizes[j])
            } else {
                (ngram_sizes[j], ngram_sizes[i])
            };
            #[allow(clippy::cast_precision_loss)]
            let max_possible = small as f64 / large as f64;
            if max_possible < threshold {
                continue;
            }

            let intersection = f1.ngram_hashes.intersection(&f2.ngram_hashes).count();

            // Exact bound check: intersection / min(s1,s2) < threshold → can't reach threshold
            #[allow(clippy::cast_precision_loss)]
            if (intersection as f64 / small as f64) < threshold {
                continue;
            }

            let union = f1.ngram_hashes.union(&f2.ngram_hashes).count();
            #[allow(clippy::cast_precision_loss)]
            let similarity = intersection as f64 / union as f64;

            if similarity >= threshold {
                advisories.push(Advisory {
                    rule_id: "REDUNDANT_BOILERPLATE".to_string(),
                    file_id: FileId(0),
                    file_path: f1.file_path.clone(),
                    severity: crate::Severity::Warning,
                    observation: {
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        let sim_pct = (similarity * 100.0) as u32;
                        format!(
                            "Redundant Boilerplate: Block '{}' is {}% similar to '{}' in {}:{}.",
                            f1.function_name, sim_pct, f2.function_name, f2.file_path, f2.line
                        )
                    },
                    impact: "Engineering Principle: Structural duplication increases technical debt and maintenance overhead.".to_string(),
                    improvement: format!("Abstract common logic shared with {}.", f2.function_name),
                    line: u32::try_from(f1.line).unwrap_or(u32::MAX),
                    column: 0,
                    start_byte: 0,
                    end_byte: 0,
                    original_content: sources
                        .get_by_path(std::path::Path::new(&f1.file_path))
                        .and_then(|src| {
                            src.content
                                .lines()
                                .nth(f1.line.saturating_sub(1))
                                .map(str::trim)
                                .map(std::string::String::from)
                        })
                        .unwrap_or_else(|| f1.function_name.clone()),
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: Some(f1.function_name.clone()),
                    #[allow(clippy::cast_possible_truncation)]
                    confidence: similarity as f32,
                    fingerprint: String::new(),
                    auto_fixable: false,
                    requires_human: true,
                    tags: vec![],
                });
            }
        }

        advisories
    }
}
