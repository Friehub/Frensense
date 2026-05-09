// SPDX-License-Identifier: MIT

pub mod consistency;
pub mod helpers;

use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::auditor::{GenSenseAuditor, ScanResult};
use super::suppression::SuppressConfig;
use crate::{parser::ParserRegistry, semantics::SymbolRegistry, Advisory, Result};

pub struct Engine {
    pub auditor: GenSenseAuditor,
    pub enabled_categories: HashSet<String>,
    pub enabled_tags: HashSet<String>,
    pub environment: crate::GenSenseEnvironment,
    pub verify_consistency: bool,
}

impl Engine {
    pub fn new(auditor: GenSenseAuditor) -> Self {
        Self {
            auditor,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            environment: crate::GenSenseEnvironment::Development,
            verify_consistency: false,
        }
    }

    pub fn set_consistency_verification(&mut self, enabled: bool) {
        self.verify_consistency = enabled;
    }

    pub fn set_environment(&mut self, env: crate::GenSenseEnvironment) {
        self.environment = env;
    }

    pub fn enable_category(&mut self, cat: &str) {
        self.enabled_categories.insert(cat.to_string());
    }

    pub fn enable_tag(&mut self, tag: &str) {
        self.enabled_tags.insert(tag.to_string());
    }

    pub fn list_rules(&self) -> Vec<(&str, &str, crate::Severity)> {
        self.auditor
            .rules()
            .iter()
            .filter(|r| {
                self.auditor.is_rule_enabled(
                    r.as_ref(),
                    &self.enabled_categories,
                    &self.enabled_tags,
                    self.environment,
                )
            })
            .map(|r| (r.id(), r.description(), r.severity()))
            .collect()
    }

    pub fn run(&mut self, root: &Path) -> Result<Vec<Advisory>> {
        let (advisories, _) = self.run_detailed(root)?;
        Ok(advisories)
    }

    pub fn run_detailed(&mut self, root: &Path) -> Result<(Vec<Advisory>, SymbolRegistry)> {
        let suppress_file = root.join(".gensense-suppress.yml");
        if suppress_file.exists() {
            if let Ok(content) = std::fs::read_to_string(suppress_file) {
                if let Ok(config) = serde_yaml::from_str::<SuppressConfig>(&content) {
                    self.auditor.set_suppressions(config);
                }
            }
        }

        let files = self.collect_files(root)?;
        let mut symbols = SymbolRegistry::new();

        let discovered: Result<Vec<Vec<crate::semantics::Symbol>>> = files
            .par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(p)?;
                self.auditor.discover_symbols(p, &content)
            })
            .collect();

        for file_symbols in discovered? {
            for sym in file_symbols {
                symbols.insert(sym);
            }
        }

        let edge_results: Result<Vec<super::auditor::FileEdges>> = files
            .par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(p)?;
                let edges = self.auditor.scan_for_edges(p, &content)?;
                Ok((p.clone(), edges))
            })
            .collect();

        for (path, file_edges) in edge_results? {
            for (caller, callee) in file_edges {
                symbols.add_call_edge(&path, &caller, &callee);
            }
        }

        // Pass 3: Event Discovery (Temporal timelines)
        // Sequential because it mutates the shared SymbolRegistry.
        for p in &files {
            let content = std::fs::read_to_string(p)?;
            self.auditor.discover_events(p, &content, &mut symbols)?;
        }

        let total_symbols = symbols.graph.graph.node_count();
        if total_symbols > 0 {
            eprintln!("[INFO] Semantic Discovery: Indexed {total_symbols} symbols across project.");
        }

        let results: Result<Vec<ScanResult>> = files
            .into_par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(&p)?;
                let (mut advisories, fingerprints) = self.auditor.audit(
                    &p,
                    &content,
                    &symbols,
                    &self.enabled_categories,
                    &self.enabled_tags,
                    self.environment,
                )?;

                if self.verify_consistency {
                    advisories.extend(self.run_consistency_analysis(&p, &content, &symbols));
                }

                for adv in &mut advisories {
                    adv.file_path = p.to_string_lossy().to_string();
                }
                Ok((advisories, fingerprints))
            })
            .collect();

        let mut all_advisories = Vec::new();
        #[cfg(feature = "fingerprinting")]
        let mut all_fingerprints = Vec::new();

        for (adv, fp) in results? {
            all_advisories.extend(adv);
            #[cfg(feature = "fingerprinting")]
            all_fingerprints.extend(fp);
        }

        #[cfg(feature = "fingerprinting")]
        all_advisories.append(&mut self.post_process_ngrams(&all_fingerprints));
        if self.enabled_tags.contains("governance") || self.enabled_tags.contains("sbom") {
            all_advisories.append(&mut self.run_governance_checks(root));
        }

        Ok((all_advisories, symbols))
    }

    pub fn run_content(&self, file_path: &Path, content: &str) -> Result<Vec<Advisory>> {
        let mut symbols = SymbolRegistry::new();
        let discovered = self.auditor.discover_symbols(file_path, content)?;
        for sym in discovered {
            symbols.insert(sym);
        }

        let (advisories, _) = self.auditor.audit(
            file_path,
            content,
            &symbols,
            &self.enabled_categories,
            &self.enabled_tags,
            self.environment,
        )?;
        Ok(advisories)
    }

    fn collect_files(&self, root: &Path) -> Result<Vec<PathBuf>> {
        if root.is_file() {
            return Ok(vec![root.to_path_buf()]);
        }
        Ok(WalkDir::new(root).into_iter().filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Ok(meta) = e.metadata() {
                    if meta.len() > 1024 * 1024 {
                        eprintln!("[WARNING] Skipping large file ({} MB): {}. Parsing large files can degrade performance.", meta.len() / 1024 / 1024, e.path().display());
                        return false;
                    }
                }
                ParserRegistry::is_supported(e.path())
            })
            .map(|e| e.path().to_path_buf())
            .collect())
    }
}
