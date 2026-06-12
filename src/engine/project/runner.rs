// SPDX-License-Identifier: MIT

use super::Engine;
use super::{FileSnapshot, cache, config};
use crate::engine::auditor::{AuditOptions, FrensenseAuditor};
use crate::engine::suppression::SuppressConfig;
use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

impl Engine {
    /// Runs the project auditor on the given root directory.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read, if configuration fails to load,
    /// or if rule execution encounters a fatal error.
    pub fn run(&mut self, root: &Path) -> Result<Vec<Advisory>> {
        let (advisories, _) = self.run_detailed(root)?;
        Ok(advisories)
    }

    /// Runs the auditor on a specific set of files (diff-only mode).
    ///
    /// Unlike `run()` which scans all files in a directory tree, this method
    /// processes only the given files. Useful with `--diff-only` to only audit
    /// changed files.
    ///
    /// # Errors
    /// Returns an error if file reading, parsing, or auditing fails.
    pub fn run_files(&mut self, root: &Path, files: &[PathBuf]) -> Result<Vec<Advisory>> {
        let _config = self.initialize_auditor_and_config(root);
        self.file_cache = cache::FileCache::load(root, self.language_filter.as_deref());

        let mut snapshots = Vec::new();
        for p in files {
            if let Some(ref allowed) = self.language_filter {
                let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                if !allowed.contains(&ext) {
                    continue;
                }
            }
            let content = match std::fs::read_to_string(p) {
                Ok(c) => c,
                Err(e) => {
                    self.file_cache.remove(p);
                    tracing::warn!("cannot read {}: {e}", p.display());
                    continue;
                }
            };
            if self.file_cache.is_unchanged(p, &content) {
                continue;
            }
            let id = self.source_registry.register(p, content.clone());
            let (language, tree) = match self.auditor.parse_source(p, &content) {
                Ok(v) => v,
                Err(e) => {
                    self.file_cache.remove(p);
                    tracing::warn!("cannot parse {}: {e}", p.display());
                    continue;
                }
            };
            let symbols = self
                .auditor
                .discover_symbols(p, id, &content, &language, &tree)?;
            let edges = self.auditor.scan_for_edges(p, &content, &language, &tree)?;
            let semantic_ops = self.auditor.extract_semantic_ops(p, &content, &tree);
            self.file_cache.update(p, &content);
            snapshots.push(FileSnapshot {
                id,
                path: p.clone(),
                content,
                tree,
                symbols,
                edges,
                semantic_ops,
            });
        }

        let mut symbols = SymbolRegistry::new();
        let mut file_ids = Vec::new();
        let mut snapshot_map = HashMap::new();
        let mut file_trees = HashMap::new();

        for snap in &snapshots {
            file_ids.push((snap.id, snap.path.clone()));
            snapshot_map.insert(snap.id, snap);
            for sym in snap.symbols.clone() {
                symbols.insert(sym);
            }
            file_trees.insert(
                snap.path.to_string_lossy().to_string(),
                (
                    snap.tree.clone(),
                    snap.content.clone(),
                    snap.semantic_ops.clone(),
                ),
            );
        }

        for snap in &snapshots {
            for (caller, callee) in &snap.edges {
                symbols.add_call_edge(&snap.path, caller, callee);
            }
        }

        for snap in &snapshots {
            self.auditor
                .discover_events(&snap.path, &snap.content, &snap.tree, &mut symbols)?;
        }

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        self.boost_overlap_confidence(&mut all_advisories);

        for snap in &snapshots {
            let mut scanner = frensense_engine::secrets::SecretScanner::new();
            scanner.add_default_patterns();
            let secret_matches = scanner.scan_tree(
                snap.tree.root_node(),
                &snap.content,
                &snap.path,
            );
            for m in secret_matches {
                all_advisories.push(crate::Advisory {
                    rule_id: format!("SECRET_{}", m.pattern_name.to_uppercase().replace(' ', "_")),
                    file_id: snap.id,
                    file_path: snap.path.to_string_lossy().to_string(),
                    severity: crate::Severity::Critical,
                    confidence: m.confidence as f32,
                    observation: format!(
                        "Potential secret found: {} ({})",
                        m.pattern_name, m.matched_text
                    ),
                    impact: "Hardcoded credentials may be exposed in source control.".to_string(),
                    improvement: "Move secrets to environment variables or a secrets manager."
                        .to_string(),
                    line: u32::try_from(m.line).unwrap_or(u32::MAX),
                    column: u32::try_from(m.column).unwrap_or(u32::MAX),
                    start_byte: u32::try_from(m.start_byte).unwrap_or(u32::MAX),
                    end_byte: u32::try_from(m.end_byte).unwrap_or(u32::MAX),
                    original_content: m.matched_text,
                    proposed_replacement: None,
                    proposed_import: None,
                    enclosing_symbol: None,
                    fingerprint: String::new(),
                    auto_fixable: false,
                    requires_human: true,
                    tags: vec!["secret".to_string(), "security".to_string()],
                });
            }
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok(all_advisories)
    }

    /// Runs the audit on a single virtual file with the given content.
    ///
    /// # Errors
    /// Returns an error if parsing or auditing fails.
    pub fn run_content(&mut self, path: &Path, content: &str) -> Result<Vec<Advisory>> {
        let config = if self.auditor.rules().is_empty() && !self.isolate_rules {
            self.initialize_auditor_and_config(Path::new("."))
        } else {
            config::load_config(Path::new("."))
        };
        let id = self.source_registry.register(path, content.to_string());
        let (language, tree) = self.auditor.parse_source(path, content)?;
        let symbols = self
            .auditor
            .discover_symbols(path, id, content, &language, &tree)?;
        let semantic_ops = self.auditor.extract_semantic_ops(path, content, &tree);

        let mut file_trees = HashMap::new();
        file_trees.insert(
            path.to_string_lossy().to_string(),
            (tree.clone(), content.to_string(), semantic_ops.clone()),
        );

        let mut registry = SymbolRegistry::new();
        for sym in symbols {
            registry.insert(sym);
        }
        self.auditor
            .discover_events(path, content, &tree, &mut registry)?;

        let opts = AuditOptions {
            file_id: id,
            path,
            content,
            tree: &tree,
            semantic_ops: &semantic_ops,
            symbols: &registry,
            graph: registry.graph(),
            file_trees: &file_trees,
            category_filter: &self.enabled_categories,
            tag_filter: &self.enabled_tags,
            suite: self.suite,
            env: self.environment,
            severity_filter: self.severity_filter,
            ngram_window_size: self.ngram_window_size,
            taint_confidence_interprocedural: self.taint_confidence_interprocedural,
            taint_confidence_intraprocedural: self.taint_confidence_intraprocedural,
            default_taint_max_depth: self.default_taint_max_depth,
        };

        let result = self.auditor.audit(&opts)?;
        let mut advisories = result.advisories;

        for rule in &self.project_rules {
            let project_advisories = rule.check_project(&registry, &self.source_registry);
            for a in project_advisories {
                if a.confidence >= self.min_confidence {
                    advisories.push(a);
                }
            }
        }

        // Merge config + CLI severity overrides (CLI wins)
        let mut merged_overrides = config.severity_override.clone().unwrap_or_default();
        for (rule_id, sev) in &self.severity_overrides {
            merged_overrides.insert(rule_id.clone(), *sev);
        }
        for adv in &mut advisories {
            if let Some(sev) = merged_overrides.get(&adv.rule_id) {
                adv.severity = *sev;
            }
        }

        // Cross-rule confidence boost
        self.boost_overlap_confidence(&mut advisories);

        Ok(advisories)
    }

    /// Boosts confidence when multiple rules fire on the same file+line.
    /// Uses `confidence_boost_rate` per overlapping rule (cap `confidence_boost_max`, max 1.0).
    fn boost_overlap_confidence(&self, advisories: &mut [Advisory]) {
        let overlap_counts: HashMap<(u32, u32), usize> = {
            let mut counts: HashMap<(u32, u32), HashSet<&str>> = HashMap::new();
            for adv in &*advisories {
                counts
                    .entry((adv.file_id.0, adv.line))
                    .or_default()
                    .insert(&adv.rule_id);
            }
            counts.into_iter().map(|(k, v)| (k, v.len())).collect()
        };

        for adv in advisories {
            if let Some(&count) = overlap_counts.get(&(adv.file_id.0, adv.line)) {
                let extra = count.saturating_sub(1);
                #[allow(clippy::cast_precision_loss)]
                let boost =
                    (extra as f32 * self.confidence_boost_rate).min(self.confidence_boost_max);
                adv.confidence = (adv.confidence + boost).min(1.0);
            }
        }
    }

    /// Runs a detailed audit, returning both advisories and the assembled symbol registry.
    ///
    /// # Errors
    /// Returns an error if file reading or parsing fails.
    #[allow(clippy::too_many_lines)]
    pub fn run_detailed(&mut self, root: &Path) -> Result<(Vec<Advisory>, SymbolRegistry)> {
        if !root.exists() {
            return Err(crate::FrensenseError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("path does not exist: {}", root.display()),
            )));
        }
        self.file_cache = cache::FileCache::load(root, self.language_filter.as_deref());
        self.cache_root = Some(root.to_path_buf());

        let config = self.initialize_auditor_and_config(root);
        let snapshots = self.collect_and_snapshot_files(root);

        let mut symbols = SymbolRegistry::new();
        let mut file_ids = Vec::new();
        let mut snapshot_map = HashMap::new();

        for snap in &snapshots {
            file_ids.push((snap.id, snap.path.clone()));
            snapshot_map.insert(snap.id, snap);
            for sym in snap.symbols.clone() {
                symbols.insert(sym);
            }
        }

        for snap in &snapshots {
            for (caller, callee) in &snap.edges {
                symbols.add_call_edge(&snap.path, caller, callee);
            }
        }

        for snap in &snapshots {
            self.auditor
                .discover_events(&snap.path, &snap.content, &snap.tree, &mut symbols)?;
        }

        let mut file_trees = HashMap::new();
        for snap in &snapshots {
            let path_str = snap.path.to_string_lossy().to_string();
            file_trees.insert(
                path_str,
                (
                    snap.tree.clone(),
                    snap.content.clone(),
                    snap.semantic_ops.clone(),
                ),
            );
        }

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        for rule in &self.project_rules {
            let project_advisories = rule.check_project(&symbols, &self.source_registry);
            for a in project_advisories {
                if a.confidence >= self.min_confidence {
                    all_advisories.push(a);
                }
            }
        }

        // Merge config + CLI severity overrides (CLI wins)
        let mut merged_overrides = config.severity_override.clone().unwrap_or_default();
        for (rule_id, sev) in &self.severity_overrides {
            merged_overrides.insert(rule_id.clone(), *sev);
        }
        for adv in &mut all_advisories {
            if let Some(sev) = merged_overrides.get(&adv.rule_id) {
                adv.severity = *sev;
            }
        }

        self.boost_overlap_confidence(&mut all_advisories);

        #[cfg(feature = "fingerprinting")]
        if let Some(ref profile) = self.profile {
            let mut all_fingerprints = Vec::new();
            for snap in &snapshots {
                let mut fps = Vec::new();
                crate::engine::fingerprint::extract_fingerprints(
                    snap.tree.root_node(),
                    &snap.content,
                    &snap.path,
                    &mut fps,
                    self.ngram_window_size,
                );
                all_fingerprints.extend(fps);
            }
            for fp in &all_fingerprints {
                let result = profile.style_surprise(fp);
                if result.score > self.profile_threshold {
                    let line = u32::try_from(fp.line).unwrap_or(u32::MAX);
                    all_advisories.push(Advisory {
                        rule_id: "STYLE_ANOMALY".to_string(),
                        file_id: FileId(0),
                        file_path: fp.file_path.clone(),
                        severity: crate::Severity::Warning,
                        #[allow(clippy::cast_possible_truncation)]
                        confidence: result.score as f32,
                        observation: format!(
                            "Style Anomaly: '{}' has {:.0}% unfamiliar patterns.",
                            fp.function_name,
                            result.score * 100.0
                        ),
                        impact: "LLM-generated code often violates project conventions — wrong casing, unfamiliar boilerplate, or types never used in this codebase."
                            .to_string(),
                        improvement:
                            "Review the function against project patterns. Consider using established conventions."
                                .to_string(),
                        line,
                        column: 0,
                        start_byte: 0,
                        end_byte: 0,
                        original_content: fp.function_name.clone(),
                        proposed_replacement: None,
                        proposed_import: None,
                        enclosing_symbol: Some(fp.function_name.clone()),
                        fingerprint: String::new(),
                        auto_fixable: false,
                        requires_human: true,
                        tags: vec![],
                    });
                }
            }

            for i in 0..all_fingerprints.len() {
                for j in (i + 1)..all_fingerprints.len() {
                    let sim = frensense_engine::minhash::approximate_jaccard(
                        &all_fingerprints[i].ngram_hashes,
                        &all_fingerprints[j].ngram_hashes,
                    );
                    if sim > 0.75 {
                        let fp_a = &all_fingerprints[i];
                        let fp_b = &all_fingerprints[j];
                        all_advisories.push(Advisory {
                            rule_id: "NEAR_DUPLICATE_FUNCTION".to_string(),
                            file_id: FileId(0),
                            file_path: fp_a.file_path.clone(),
                            severity: crate::Severity::Info,
                            confidence: sim as f32,
                            observation: format!(
                                "Near-duplicate function: '{}' in {} (line {}) is {:.0}% similar to '{}' in {} (line {}).",
                                fp_a.function_name,
                                fp_a.file_path,
                                fp_a.line,
                                sim * 100.0,
                                fp_b.function_name,
                                fp_b.file_path,
                                fp_b.line,
                            ),
                            impact: "Copy-pasted code diverges over time — one copy may lack security fixes."
                                .to_string(),
                            improvement: "Consider extracting shared logic into a common function."
                                .to_string(),
                            line: u32::try_from(fp_a.line).unwrap_or(u32::MAX),
                            column: 0,
                            start_byte: 0,
                            end_byte: 0,
                            original_content: fp_a.function_name.clone(),
                            proposed_replacement: None,
                            proposed_import: None,
                            enclosing_symbol: Some(fp_a.function_name.clone()),
                            fingerprint: String::new(),
                            auto_fixable: false,
                            requires_human: true,
                            tags: vec!["copy-paste".to_string(), "duplicate".to_string()],
                        });
                    }
                }
            }
        }

        if let Some(ref corpus_dir) = self.corpus_dir {
            let mut registry = frensense_engine::corpus::registry::PatternRegistry::new(0.60);
            match registry.load_corpus(corpus_dir) {
                Ok(count) if count > 0 => {
                    for snap in &snapshots {
                        let mut fps: Vec<frensense_engine::fingerprint::FunctionFingerprint> = Vec::new();
                        frensense_engine::fingerprint::extract_fingerprints(
                            snap.tree.root_node(),
                            &snap.content,
                            &snap.path,
                            &mut fps,
                            self.ngram_window_size,
                        );
                        for fp in &fps {
                            for m in registry.scan_function(fp) {
                                all_advisories.push(Advisory {
                                    rule_id: format!("CORPUS_{}", m.pattern_id.to_uppercase()),
                                    file_id: snap.id,
                                    file_path: snap.path.to_string_lossy().to_string(),
                                    severity: crate::Severity::Warning,
                                    confidence: m.score as f32,
                                    observation: format!(
                                        "Corpus pattern: {} (score {:.2}) in '{}'",
                                        m.pattern_id, m.score, fp.function_name,
                                    ),
                                    impact: "Function shape matches a known violation pattern."
                                        .to_string(),
                                    improvement: "Review against corpus example.".to_string(),
                                    line: u32::try_from(fp.line).unwrap_or(u32::MAX),
                                    column: 0,
                                    start_byte: 0,
                                    end_byte: 0,
                                    original_content: fp.function_name.clone(),
                                    proposed_replacement: None,
                                    proposed_import: None,
                                    enclosing_symbol: Some(fp.function_name.clone()),
                                    fingerprint: String::new(),
                                    auto_fixable: false,
                                    requires_human: true,
                                    tags: vec!["corpus".to_string(), "pattern".to_string()],
                                });
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => eprintln!("Corpus load error: {e}"),
            }
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok((all_advisories, symbols))
    }

    fn initialize_auditor_and_config(&mut self, root: &Path) -> config::FrensenseConfig {
        let config = config::load_config(root);

        if !self.isolate_rules {
            let mut dirs = self.extra_rule_dirs.clone();
            if let Some(config_rules_dir) = &config.rules_dir {
                dirs.push(PathBuf::from(config_rules_dir));
            }
            let (rules, project_rules) =
                FrensenseAuditor::build_rule_set(root, &dirs, self.no_builtin_rules);
            self.auditor.set_rules(rules);
            self.project_rules = project_rules;

            let mut disabled_set: HashSet<&str> = HashSet::new();
            if let Some(disabled) = &config.disabled_rules {
                for id in disabled {
                    disabled_set.insert(id.as_str());
                }
            }
            for id in &self.disabled_rule_ids {
                disabled_set.insert(id.as_str());
            }
            if !disabled_set.is_empty() {
                self.auditor
                    .retain_rules(|r| !disabled_set.contains(r.id()));
                self.project_rules
                    .retain(|r| !disabled_set.contains(r.metadata().id.as_ref()));
            }

            if let Some(_max_lines) = self.max_source_lines {
            }
        } else if !self.extra_rule_dirs.is_empty() {
            let (user_rules, user_project_rules) =
                crate::engine::auditor::user_rules::load_user_rules(root, &self.extra_rule_dirs);
            self.auditor.add_rules(user_rules);
            self.project_rules.extend(user_project_rules);

            if let Some(_max_lines) = self.max_source_lines {
            }
        } else if let Some(_max_lines) = self.max_source_lines {
        }

        let suppress_file = root.join(".frensense-suppress.yml");
        if suppress_file.exists()
            && let Ok(content) = std::fs::read_to_string(suppress_file)
            && let Ok(supp_config) = serde_yaml::from_str::<SuppressConfig>(&content)
        {
            self.auditor.set_suppressions(supp_config);
        }
        config
    }

    fn collect_and_snapshot_files(&mut self, root: &Path) -> Vec<FileSnapshot> {
        super::files::collect_files_impl(self, root)
    }

    /// Collects all files reachable from `root` that match supported extensions
    /// and the optional language filter.
    #[must_use]
    pub fn collect_files(root: &Path, language_filter: Option<&Vec<&'static str>>) -> Vec<PathBuf> {
        super::files::collect_files(root, language_filter)
    }

    fn perform_parallel_audit(
        &self,
        file_ids: &[(FileId, PathBuf)],
        snapshot_map: &HashMap<FileId, &FileSnapshot>,
        symbols: &mut SymbolRegistry,
        file_trees: &HashMap<
            String,
            (
                tree_sitter::Tree,
                String,
                Vec<crate::semantics::data_flow::normalization::SemanticOp>,
            ),
        >,
    ) -> Result<Vec<Advisory>> {
        super::files::parallel_audit_impl(self, file_ids, snapshot_map, symbols, file_trees)
    }
}
