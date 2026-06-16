// SPDX-License-Identifier: MIT

use super::Engine;
use super::{FileSnapshot, cache, config};
use crate::engine::auditor::AuditOptions;
use crate::engine::suppression::SuppressConfig;
use crate::semantics::data_flow::TaintRegistry;
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

        tracing::debug!("run_taint_analysis: starting");
        for rule in crate::engine::taint_rules::load_all_taint_rules(&self.extra_taint_rule_dirs) {
            let source_re = regex::Regex::new(&rule.source_re).ok();
            let sink_re = regex::Regex::new(&rule.sink_re).ok();
            if source_re.is_none() || sink_re.is_none() {
                continue;
            }
            let source = source_re.unwrap();
            let sink = sink_re.unwrap();

            for snap in &snapshots {
                let context = crate::FrensenseContext {
                    file_id: snap.id,
                    file_path: &snap.path,
                    source_code: &snap.content,
                    tree: &snap.tree,
                    symbols: &symbols,
                    graph: symbols.graph(),
                    semantic_ops: &snap.semantic_ops,
                    taint_cache: &crate::TaintCache::new(),
                    file_trees: &file_trees,
                    taint_confidence_interprocedural: self.taint_confidence_interprocedural,
                    taint_confidence_intraprocedural: self.taint_confidence_intraprocedural,
                    default_taint_max_depth: self.default_taint_max_depth,
                    ngram_window_size: self.ngram_window_size,
                };

                let root = snap.tree.root_node();
                let analyzer = crate::semantics::data_flow::DataFlowAnalyzer::new(&context, root);
                let mut registry = TaintRegistry::default();
                analyzer.discover_symbols(&mut registry);

                let functions: Vec<tree_sitter::Node> = collect_function_nodes(root);
                tracing::debug!(
                    "taint: rule={} file={} fn_count={} ops_count={}",
                    rule.id,
                    snap.path.display(),
                    functions.len(),
                    snap.semantic_ops.len(),
                );
                for fn_node in &functions {
                    let body = fn_node.child_by_field_name("body").unwrap_or(*fn_node);
                    let fn_name = &snap.content[fn_node.start_byte()..fn_node.end_byte()];
                    let metrics = frensense_engine::data_flow::taint_metrics::TaintMetrics::compute(
                        &registry,
                        body,
                        &snap.content,
                        fn_name,
                    );
                    let findings = analyzer.analyze_block(
                        body,
                        &source,
                        &sink,
                        &MinimalRule {
                            id: rule.id.clone(),
                            severity: rule.severity,
                            impact: rule.impact.clone(),
                            improvement: rule.improvement.clone(),
                        },
                        &mut registry,
                    );

                    for mut adv in findings {
                        let adjusted = frensense_engine::data_flow::confidence::TaintConfidenceAdjuster::adjust_confidence(
                            &snap.content,
                            &snap.path,
                            adv.line,
                            &adv.original_content,
                            adv.confidence,
                        );
                        adv.confidence = adjusted;
                        if metrics.is_hollow_validator() {
                            adv.confidence = (adv.confidence * 0.4).max(0.15);
                        }
                        all_advisories.push(adv);
                    }
                }
            }
        }

        self.boost_overlap_confidence(&mut all_advisories);

        for snap in &snapshots {
            let mut scanner = frensense_engine::secrets::SecretScanner::new();
            scanner.add_default_patterns();
            let secret_matches =
                scanner.scan_tree(snap.tree.root_node(), &snap.content, &snap.path);
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

        // W7: Dependency hallucination check
        let mut dep_resolver = frensense_engine::deps::DependencyResolver::new();
        dep_resolver.load_project(root);
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::hallucinated_import::find(
                &mut dep_resolver,
                snap,
            ));
        }

        // W2: Dead branch detection
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::dead_branch::find(snap));
        }

        // W3: Unused variables via def-use
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::unused_variable::find(snap));
        }

        // W1: Temporal violations
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::temporal_violation::find(snap));
        }

        // W4: Cross-file taint
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::cross_file_taint::find(
                &symbols, snap,
            ));
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok(all_advisories)
    }

    /// Runs the audit on a single virtual file with the given content.
    ///
    /// # Errors
    /// Returns an error if parsing or auditing fails.
    pub fn run_content(&mut self, path: &Path, content: &str) -> Result<Vec<Advisory>> {
        let config = if self.auditor.rules().is_empty() {
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

        // Collect all corpus dirs
        let mut corpus_dirs: Vec<&std::path::Path> = Vec::new();
        if let Some(ref corpus_dir) = self.corpus_dir {
            corpus_dirs.push(corpus_dir.as_path());
        }

        if !corpus_dirs.is_empty() {
            let mut all_metadata = HashMap::new();
            for dir in &corpus_dirs {
                all_metadata.extend(load_corpus_metadata(dir));
            }
            let mut registry = frensense_engine::corpus::registry::PatternRegistry::new(0.60);
            match registry.load_corpus_dirs(&corpus_dirs) {
                Ok(count) if count > 0 => {
                    for snap in &snapshots {
                        let mut fps: Vec<frensense_engine::fingerprint::FunctionFingerprint> =
                            Vec::new();
                        frensense_engine::fingerprint::extract_fingerprints(
                            snap.tree.root_node(),
                            &snap.content,
                            &snap.path,
                            &mut fps,
                            self.ngram_window_size,
                        );
                        for fp in &fps {
                            for m in registry.scan_function(fp) {
                                let meta = all_metadata.get(&m.pattern_id);
                                all_advisories.push(Advisory {
                                    rule_id: meta
                                        .and_then(|md| md.get("id"))
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            format!("CORPUS_{}", m.pattern_id.to_uppercase())
                                        }),
                                    file_id: snap.id,
                                    file_path: snap.path.to_string_lossy().to_string(),
                                    severity: meta
                                        .and_then(|md| md.get("severity"))
                                        .map(|s| match s.to_lowercase().as_str() {
                                            "critical" => crate::Severity::Critical,
                                            "warning" => crate::Severity::Warning,
                                            _ => crate::Severity::Info,
                                        })
                                        .unwrap_or(crate::Severity::Warning),
                                    confidence: m.score as f32,
                                    observation: meta
                                        .and_then(|md| md.get("observation"))
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            format!(
                                                "Corpus pattern: {} (score {:.2}) in '{}'",
                                                m.pattern_id, m.score, fp.function_name,
                                            )
                                        }),
                                    impact: meta
                                        .and_then(|md| md.get("impact"))
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            "Function shape matches a known violation pattern."
                                                .to_string()
                                        }),
                                    improvement: meta
                                        .and_then(|md| md.get("improvement"))
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            "Review against corpus example.".to_string()
                                        }),
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

        self.run_taint_analysis(&snapshots, &symbols, &file_trees, &mut all_advisories);

        // W4: Cross-file taint (via findings module)
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::cross_file_taint::find(
                &symbols, snap,
            ));
        }

        // W7: Dependency hallucination check
        let mut dep_resolver = frensense_engine::deps::DependencyResolver::new();
        dep_resolver.load_project(root);
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::hallucinated_import::find(
                &mut dep_resolver,
                snap,
            ));
        }

        // W2: Dead branch detection
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::dead_branch::find(snap));
        }

        // W3: Unused variables via def-use
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::unused_variable::find(snap));
        }

        // W1: Temporal violations
        for snap in &snapshots {
            all_advisories.extend(crate::engine::findings::temporal_violation::find(snap));
        }

        // Apply severity overrides to taint findings too
        for adv in &mut all_advisories {
            if let Some(sev) = merged_overrides.get(&adv.rule_id) {
                adv.severity = *sev;
            }
        }

        if let Some(ref baseline_path) = self.baseline_path {
            if let Ok(prev) = std::fs::read_to_string(baseline_path) {
                if let Ok(fingerprints) = serde_json::from_str::<Vec<String>>(&prev) {
                    let baseline_set: std::collections::HashSet<String> =
                        fingerprints.into_iter().collect();
                    all_advisories.retain(|a| !baseline_set.contains(&a.fingerprint));
                }
            }
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok((all_advisories, symbols))
    }

    fn run_taint_analysis(
        &self,
        snapshots: &[super::FileSnapshot],
        symbols: &SymbolRegistry,
        file_trees: &std::collections::HashMap<
            String,
            (
                tree_sitter::Tree,
                String,
                Vec<crate::semantics::data_flow::normalization::SemanticOp>,
            ),
        >,
        all_advisories: &mut Vec<Advisory>,
    ) {
        for rule in crate::engine::taint_rules::load_all_taint_rules(&self.extra_taint_rule_dirs) {
            let Some(source) = regex::Regex::new(&rule.source_re).ok() else {
                continue;
            };
            let Some(sink) = regex::Regex::new(&rule.sink_re).ok() else {
                continue;
            };

            for snap in snapshots {
                let context = crate::FrensenseContext {
                    file_id: snap.id,
                    file_path: &snap.path,
                    source_code: &snap.content,
                    tree: &snap.tree,
                    symbols,
                    graph: symbols.graph(),
                    semantic_ops: &snap.semantic_ops,
                    taint_cache: &crate::TaintCache::new(),
                    file_trees,
                    taint_confidence_interprocedural: self.taint_confidence_interprocedural,
                    taint_confidence_intraprocedural: self.taint_confidence_intraprocedural,
                    default_taint_max_depth: self.default_taint_max_depth,
                    ngram_window_size: self.ngram_window_size,
                };

                let root = snap.tree.root_node();
                let analyzer = crate::semantics::data_flow::DataFlowAnalyzer::new(&context, root);
                let mut registry = TaintRegistry::default();
                analyzer.discover_symbols(&mut registry);

                for fn_node in &collect_function_nodes(root) {
                    let body = fn_node.child_by_field_name("body").unwrap_or(*fn_node);
                    let fn_name = &snap.content[fn_node.start_byte()..fn_node.end_byte()];
                    let metrics = frensense_engine::data_flow::taint_metrics::TaintMetrics::compute(
                        &registry,
                        body,
                        &snap.content,
                        fn_name,
                    );
                    let findings = analyzer.analyze_block(
                        body,
                        &source,
                        &sink,
                        &MinimalRule {
                            id: rule.id.clone(),
                            severity: rule.severity,
                            impact: rule.impact.clone(),
                            improvement: rule.improvement.clone(),
                        },
                        &mut registry,
                    );

                    for mut adv in findings {
                        adv.confidence = frensense_engine::data_flow::confidence::TaintConfidenceAdjuster::adjust_confidence(
                            &snap.content,
                            &snap.path,
                            adv.line,
                            &adv.original_content,
                            adv.confidence,
                        );
                        if metrics.is_hollow_validator() {
                            adv.confidence = (adv.confidence * 0.4).max(0.15);
                        }
                        all_advisories.push(adv);
                    }
                }
            }
        }
    }

    fn initialize_auditor_and_config(&mut self, root: &Path) -> config::FrensenseConfig {
        let config = config::load_config(root);

        // Apply disabled_rules from config + CLI
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
        }

        // Load suppressions
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

fn load_corpus_metadata(corpus_dir: &Path) -> HashMap<String, HashMap<String, String>> {
    let mut metadata = HashMap::new();
    let Ok(entries) = std::fs::read_dir(corpus_dir) else {
        return metadata;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(table) = content.parse::<toml::Table>() else {
            continue;
        };
        let mut map = HashMap::new();
        for (k, v) in &table {
            if let Some(s) = v.as_str() {
                map.insert(k.clone(), s.to_string());
            }
        }
        metadata.insert(stem.to_string(), map);
    }
    metadata
}

fn collect_function_nodes(node: tree_sitter::Node) -> Vec<tree_sitter::Node> {
    let mut functions = Vec::new();
    let kind = node.kind();
    if matches!(
        kind,
        "function_item"
            | "function_declaration"
            | "method_definition"
            | "arrow_function"
            | "function_definition"
    ) {
        functions.push(node);
    }
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            functions.extend(collect_function_nodes(cursor.node()));
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    functions
}

struct MinimalRule {
    id: String,
    severity: crate::Severity,
    impact: String,
    improvement: String,
}

impl crate::FrensenseRule for MinimalRule {
    fn metadata(&self) -> &crate::RuleMetadata {
        Box::leak(Box::new(crate::RuleMetadata {
            id: std::borrow::Cow::Owned(self.id.clone()),
            name: std::borrow::Cow::Owned(self.id.clone()),
            severity: self.severity,
            observation: std::borrow::Cow::Borrowed(""),
            impact: std::borrow::Cow::Owned(self.impact.clone()),
            improvement: std::borrow::Cow::Owned(self.improvement.clone()),
            tags: Vec::new(),
            category: std::borrow::Cow::Borrowed("security"),
            confidence: 0.85,
            precision: crate::Precision::High,
        }))
    }

    fn check<'a>(
        &self,
        _node: tree_sitter::Node<'a>,
        _context: &crate::FrensenseContext<'a>,
    ) -> Vec<crate::Advisory> {
        Vec::new()
    }

    fn applies_to(&self, _extension: &str) -> bool {
        true
    }
}
