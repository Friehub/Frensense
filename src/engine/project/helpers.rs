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
    pub fn post_process_ngrams(
        &self,
        fingerprints: &[FunctionFingerprint],
        sources: &SourceRegistry,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        let mut similarity_map: std::collections::HashMap<u64, Vec<usize>> =
            std::collections::HashMap::new();
        for (idx, fp) in fingerprints.iter().enumerate() {
            for &hash in &fp.ngram_hashes {
                similarity_map.entry(hash).or_default().push(idx);
            }
        }

        let mut compared = std::collections::HashSet::new();
        for (i, f1) in fingerprints.iter().enumerate() {
            let mut candidates = std::collections::HashSet::new();
            for &hash in &f1.ngram_hashes {
                if let Some(indices) = similarity_map.get(&hash) {
                    for &j in indices {
                        if j > i {
                            candidates.insert(j);
                        }
                    }
                }
            }
            for j in candidates {
                if !compared.insert((i, j)) {
                    continue;
                }
                let f2 = &fingerprints[j];
                let intersection = f1.ngram_hashes.intersection(&f2.ngram_hashes).count();
                let union = f1.ngram_hashes.union(&f2.ngram_hashes).count();
                #[allow(clippy::cast_precision_loss)]
                let similarity = intersection as f64 / union as f64;
                if similarity >= 0.8 {
                    advisories.push(Advisory {
                        rule_id: "REDUNDANT_BOILERPLATE".to_string(),
                        file_id: FileId(0), // Global finding
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
        }
        advisories
    }
}
