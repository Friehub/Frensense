// SPDX-License-Identifier: MIT

pub mod config;
pub mod consistency;
pub mod helpers;

use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::auditor::{GenSenseAuditor, ScanResult};
use super::source::SourceRegistry;
use super::suppression::SuppressConfig;
use crate::{parser::ParserRegistry, semantics::SymbolRegistry, Advisory, Result};

pub struct Engine {
    pub auditor: GenSenseAuditor,
    pub source_registry: SourceRegistry,
    pub enabled_categories: HashSet<String>,
    pub enabled_tags: HashSet<String>,
    pub environment: crate::GenSenseEnvironment,
    pub verify_consistency: bool,
    pub extra_rule_dirs: Vec<PathBuf>,
    pub no_builtin_rules: bool,
    pub isolate_rules: bool,
}

impl Engine {
    pub fn new(auditor: GenSenseAuditor) -> Self {
        Self {
            auditor,
            source_registry: SourceRegistry::new(),
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            environment: crate::GenSenseEnvironment::Development,
            verify_consistency: false,
            extra_rule_dirs: Vec::new(),
            no_builtin_rules: false,
            isolate_rules: false,
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

    pub fn list_rules(&self) -> Vec<(String, String, crate::Severity)> {
        self.auditor
            .rules()
            .iter()
            .map(|r| {
                let meta = r.metadata();
                (meta.id.to_string(), meta.impact.to_string(), meta.severity)
            })
            .collect()
    }

    pub fn run(&mut self, root: &Path) -> Result<Vec<Advisory>> {
        let (advisories, _) = self.run_detailed(root)?;
        Ok(advisories)
    }

    pub fn run_detailed(&mut self, root: &Path) -> Result<(Vec<Advisory>, SymbolRegistry)> {
        let config = config::load_config(root);

        if !self.isolate_rules {
            let mut dirs = self.extra_rule_dirs.clone();
            if let Some(config_rules_dir) = &config.rules_dir {
                dirs.push(PathBuf::from(config_rules_dir));
            }
            self.auditor.rules =
                GenSenseAuditor::build_rule_set(root, &dirs, self.no_builtin_rules);
            if let Some(disabled) = &config.disabled_rules {
                let disabled_set: HashSet<&str> = disabled.iter().map(|s| s.as_str()).collect();
                self.auditor
                    .rules
                    .retain(|r| !disabled_set.contains(r.id()));
            }
        }

        let suppress_file = root.join(".gensense-suppress.yml");
        if suppress_file.exists() {
            if let Ok(content) = std::fs::read_to_string(suppress_file) {
                if let Ok(config) = serde_yaml::from_str::<SuppressConfig>(&content) {
                    self.auditor.set_suppressions(config);
                }
            }
        }

        let files = self.collect_files(root)?;

        // Pass 1: Parallel Mapping (Immutable Snapshots)
        // Each file is processed in isolation, producing its local semantic snapshots.
        #[derive(Debug)]
        struct FileSnapshot {
            path: PathBuf,
            content: String,
            symbols: Vec<crate::semantics::Symbol>,
            edges: Vec<(String, String)>,
        }

        let snapshots: Result<Vec<FileSnapshot>> = files
            .into_par_iter()
            .map(|p| {
                let content = std::fs::read_to_string(&p)?;
                let symbols = self.auditor.discover_symbols(&p, &content)?;
                let edges = self.auditor.scan_for_edges(&p, &content)?;
                Ok(FileSnapshot {
                    path: p,
                    content,
                    symbols,
                    edges,
                })
            })
            .collect();

        // Pass 2: Sequential Assembly (Converging snapshots into the dependency graph)
        let mut symbols = SymbolRegistry::new();
        let mut file_ids = Vec::new();

        let snapshots = snapshots?;
        for snap in &snapshots {
            let id = self
                .source_registry
                .register(&snap.path, snap.content.clone());
            file_ids.push((id, snap.path.clone()));
            for sym in snap.symbols.clone() {
                symbols.insert(sym);
            }
        }

        // Pass 3: Dependency Connection (Connecting the snapshots)
        for snap in &snapshots {
            for (caller, callee) in &snap.edges {
                symbols.add_call_edge(&snap.path, caller, callee);
            }
        }

        // Pass 4: Parallel Audit (Read-only access to the assembled graph)
        let results: Result<Vec<ScanResult>> = file_ids
            .into_par_iter()
            .map(|(id, p)| {
                let source = self.source_registry.get(id).unwrap();
                let (mut advisories, fingerprints) = self.auditor.audit(
                    id,
                    &p,
                    &source.content,
                    &symbols,
                    &self.enabled_categories,
                    &self.enabled_tags,
                    self.environment,
                )?;

                if self.verify_consistency {
                    advisories.extend(self.run_consistency_analysis(
                        id,
                        &p,
                        &source.content,
                        &symbols,
                    ));
                }

                Ok((advisories, fingerprints))
            })
            .collect();

        let mut all_advisories = Vec::new();
        for (adv, _) in results? {
            all_advisories.extend(adv);
        }

        if let Some(overrides) = &config.severity_override {
            for adv in &mut all_advisories {
                if let Some(sev) = overrides.get(&adv.rule_id) {
                    adv.severity = *sev;
                }
            }
        }

        Ok((all_advisories, symbols))
    }

    fn collect_files(&self, root: &Path) -> Result<Vec<PathBuf>> {
        if root.is_file() {
            return Ok(vec![root.to_path_buf()]);
        }
        Ok(WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                if e.file_type().is_dir() {
                    let name = e.file_name().to_string_lossy();
                    if e.path() != root {
                        return name != "target"
                            && name != "node_modules"
                            && !name.starts_with('.');
                    }
                }
                true
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                if let Ok(meta) = e.metadata() {
                    if meta.len() > 1024 * 1024 {
                        return false;
                    }
                }
                ParserRegistry::is_supported(e.path())
            })
            .map(|e| e.path().to_path_buf())
            .collect())
    }

    pub fn run_content(&self, file_path: &Path, content: &str) -> Result<Vec<Advisory>> {
        let mut registry = SourceRegistry::new();
        let id = registry.register(file_path, content.to_string());
        let mut symbols = SymbolRegistry::new();
        let discovered = self.auditor.discover_symbols(file_path, content)?;
        for sym in discovered {
            symbols.insert(sym);
        }

        let (advisories, _) = self.auditor.audit(
            id,
            file_path,
            content,
            &symbols,
            &self.enabled_categories,
            &self.enabled_tags,
            self.environment,
        )?;
        Ok(advisories)
    }
}
