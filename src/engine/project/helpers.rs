// SPDX-License-Identifier: MIT

use super::super::fingerprint::FunctionFingerprint;
use super::Engine;
use crate::{Advisory, FileId};
use std::path::Path;

impl Engine {
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
                original_content: String::new(),
                proposed_replacement: None,
                proposed_import: None,
                enclosing_symbol: None,
            });
        }
        advisories
    }

    #[cfg(feature = "fingerprinting")]
    pub fn post_process_ngrams(&self, fingerprints: &[FunctionFingerprint]) -> Vec<Advisory> {
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
                let similarity = intersection as f64 / union as f64;
                if similarity >= 0.8 {
                    advisories.push(Advisory {
                        rule_id: "REDUNDANT_BOILERPLATE".to_string(),
                        file_id: FileId(0), // Global finding
                        file_path: f1.file_path.clone(),
                        severity: crate::Severity::Warning,
                        observation: format!(
                            "Redundant Boilerplate: Block '{}' is {}% similar to '{}' in {}:{}.",
                            f1.function_name, (similarity * 100.0) as u32, f2.function_name, f2.file_path, f2.line
                        ),
                        impact: "Engineering Principle: Structural duplication increases technical debt and maintenance overhead.".to_string(),
                        improvement: format!("Abstract common logic shared with {}.", f2.function_name),
                        line: f1.line as u32,
                        column: 0,
                        start_byte: 0,
                        end_byte: 0,
                        original_content: String::new(),
                        proposed_replacement: None,
                        proposed_import: None,
                        enclosing_symbol: Some(f1.function_name.clone()),
                    });
                }
            }
        }
        advisories
    }
}
