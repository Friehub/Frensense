// SPDX-License-Identifier: MIT

use super::Engine;
use super::{FileSnapshot, cache, config};
use crate::engine::auditor::AuditOptions;
use crate::engine::suppression::SuppressConfig;

use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Shared snapshot processing: build symbol registry, add edges, discover events.
fn process_snapshots<'a>(
    auditor: &crate::engine::auditor::FrensenseAuditor,
    snapshots: &'a [FileSnapshot],
) -> Result<(SymbolRegistry, Vec<(FileId, PathBuf)>, HashMap<FileId, &'a FileSnapshot>)> {
    let mut symbols = SymbolRegistry::new();
    let mut file_ids = Vec::new();
    let mut snapshot_map = HashMap::new();

    for snap in snapshots {
        file_ids.push((snap.id, snap.path.clone()));
        snapshot_map.insert(snap.id, snap);
        for sym in snap.symbols.clone() {
            symbols.insert(sym);
        }
    }

    for snap in snapshots {
        for (caller, callee) in &snap.edges {
            symbols.add_call_edge(&snap.path, caller, callee);
        }
    }

    for snap in snapshots {
        auditor.discover_events(&snap.path, &snap.content, &snap.tree, &mut symbols)?;
    }

    Ok((symbols, file_ids, snapshot_map))
}

/// Build file_trees map from snapshots.
fn build_file_trees(snapshots: &[FileSnapshot]) -> HashMap<String, (tree_sitter::Tree, String, Vec<crate::semantics::data_flow::normalization::SemanticOp>)> {
    let mut file_trees = HashMap::new();
    for snap in snapshots {
        file_trees.insert(
            snap.path.to_string_lossy().to_string(),
            (snap.tree.clone(), snap.content.clone(), snap.semantic_ops.clone()),
        );
    }
    file_trees
}

/// Merge config + CLI severity overrides (CLI wins) into advisories.
fn apply_severity_overrides(advisories: &mut [Advisory], config_overrides: &Option<HashMap<String, crate::Severity>>, cli_overrides: &HashMap<String, crate::Severity>) {
    let mut merged = config_overrides.clone().unwrap_or_default();
    for (rule_id, sev) in cli_overrides {
        merged.insert(rule_id.clone(), *sev);
    }
    for adv in advisories {
        if let Some(sev) = merged.get(&adv.rule_id) {
            adv.severity = *sev;
        }
    }
}

/// Run all findings modules (W1-W7) on snapshots.
fn run_findings_modules(
    root: &Path,
    snapshots: &[FileSnapshot],
    symbols: &SymbolRegistry,
    _file_trees: &HashMap<String, (tree_sitter::Tree, String, Vec<crate::semantics::data_flow::normalization::SemanticOp>)>,
    _extra_taint_rule_dirs: &[PathBuf],
    check_deps: bool,
    all_advisories: &mut Vec<Advisory>,
) {
    use crate::engine::findings::{FindingContext, registered_modules};

    let modules = registered_modules();

    // Setup shared state needed by some modules
    let mut dep_resolver = frensense_engine::deps::DependencyResolver::with_check_deps(check_deps);
    dep_resolver.load_project(root);

    // Create a DataFlowEngine for cross-file taint analysis
    let data_flow_engine = frensense_engine::data_flow::DataFlowEngine::new();

    for snap in snapshots {
        let mut ctx = FindingContext {
            symbols,
            dep_resolver: Some(&mut dep_resolver),
            data_flow_engine: Some(&data_flow_engine),
        };
        for module in &modules {
            all_advisories.extend(module.run(snap, &mut ctx));
        }
    }
}

/// Run corpus pattern matching on snapshots.
fn run_corpus_scan(
    engine: &Engine,
    _root: &Path,
    snapshots: &[FileSnapshot],
    symbols: &crate::semantics::symbols::SymbolRegistry,
    data_flow: &frensense_engine::data_flow::DataFlowEngine,
    file_trees: &HashMap<String, (tree_sitter::Tree, String, Vec<crate::semantics::data_flow::normalization::SemanticOp>)>,
    all_advisories: &mut Vec<Advisory>,
) {
    let mut corpus_dirs: Vec<&Path> = Vec::new();
    if let Some(ref corpus_dir) = engine.corpus_dir {
        corpus_dirs.push(corpus_dir.as_path());
    }

    let mut registry = frensense_engine::corpus::registry::PatternRegistry::new(engine.corpus_threshold);
    for (category, threshold) in &engine.threshold_overrides {
        registry.set_threshold_override(category.clone(), *threshold);
    }
    let mut corpus_loaded = false;

    #[cfg(feature = "fingerprinting")]
    if let Some(bundle_bytes) = engine.corpus_bundle {
        match registry.load_from_bundle(bundle_bytes) {
            Ok(count) if count > 0 => {
                eprintln!("Loaded {count} patterns from embedded bundle");
                corpus_loaded = true;
            }
            Ok(_) => {}
            Err(e) => eprintln!("Bundle load error: {e}"),
        }
    }

    // Also load from corpus directories if specified (adds to embedded bundle)
    if !corpus_dirs.is_empty() {
        match registry.load_corpus_dirs(&corpus_dirs) {
            Ok(count) if count > 0 => {
                eprintln!("Loaded {count} patterns from corpus directory");
                corpus_loaded = true;
            }
            Ok(_) => {}
            Err(e) => eprintln!("Corpus load error: {e}"),
        }
    }

    if !corpus_loaded {
        return;
    }

    for snap in snapshots {
        let mut fps = Vec::new();
        frensense_engine::fingerprint::extract_fingerprints(
            snap.tree.root_node(), &snap.content, &snap.path, &mut fps, engine.ngram_window_size,
        );
        for fp in &fps {
            let func_node = find_function_node(snap.tree.root_node(), &fp.function_name, fp.line, &snap.content);
            for m in registry.scan_function(fp, func_node, Some(&snap.content)) {
                let impact = m.impact.unwrap_or_else(|| "Function shape matches a known violation pattern.".to_string());
                let improvement = m.improvement.unwrap_or_else(|| "Review against corpus example.".to_string());
                let observation = m.observation.unwrap_or_else(|| {
                    format!("Corpus pattern: {} (score {:.2}) in '{}'", m.pattern_id, m.score, fp.function_name)
                });

                // Apply confidence calibration if available
                // Extract category from pattern ID for per-category calibration
                let category = m.pattern_id.split('_').nth(1).unwrap_or("default");
                let mut confidence = if let Some(ref per_cat_cal) = engine.per_category_calibration {
                    per_cat_cal.calibrate(m.score, category) as f32
                } else if let Some(ref params) = engine.calibration {
                    params.calibrate(m.score) as f32
                } else {
                    m.score as f32
                };

                // Verify taint flow if we have a function node
                let mut taint_verified = false;
                let mut taint_detail = String::new();
                if let Some(fn_node) = func_node {
                    let verification = verify_taint_flow(
                        fn_node,
                        &snap.content,
                        &snap.tree,
                        &snap.path,
                        &symbols,
                        &data_flow,
                        &file_trees,
                    );
                    if verification.verified {
                        taint_verified = true;
                        taint_detail = verification.detail;
                        // Boost confidence for verified findings
                        confidence = (confidence * 1.2).min(0.95);
                    }
                }

                let mut advisory = Advisory::bare(
                    format!("CORPUS_{}", m.pattern_id.to_uppercase()),
                    crate::Severity::Warning,
                    snap.id,
                    &snap.path,
                    &observation,
                )
                .with_confidence(confidence)
                .with_line(u32::try_from(fp.line).unwrap_or(u32::MAX))
                .with_content(fp.function_name.clone())
                .with_enclosing_symbol(fp.function_name.clone())
                .with_impact(&impact)
                .with_improvement(&improvement)
                .with_tags(["corpus", "pattern"]);

                // Add taint verification info if available
                if taint_verified {
                    advisory = advisory.with_tags(["corpus", "pattern", "taint-verified"]);
                    advisory.impact = format!("{}\n\nTaint flow verified: {}", impact, taint_detail);
                }

                all_advisories.push(advisory);
            }
        }
    }
}

/// Verification result from taint flow analysis.
struct TaintVerification {
    verified: bool,
    detail: String,
}

/// Verify that taint actually flows from source to sink in a function.
///
/// This uses the InterproceduralVerifier to check if user-controlled data
/// reaches a dangerous sink, following taint through function calls.
fn verify_taint_flow(
    fn_node: tree_sitter::Node,
    source: &str,
    tree: &tree_sitter::Tree,
    file_path: &Path,
    symbols: &crate::semantics::symbols::SymbolRegistry,
    data_flow: &frensense_engine::data_flow::DataFlowEngine,
    file_trees: &HashMap<String, (tree_sitter::Tree, String, Vec<crate::semantics::data_flow::normalization::SemanticOp>)>,
) -> TaintVerification {
    use crate::semantics::data_flow::cross_file::CrossFileVerifier;

    let file_path_str = file_path.to_string_lossy().to_string();
    let mut verifier = CrossFileVerifier::new(
        source,
        tree,
        &file_path_str,
        symbols,
        data_flow,
        file_trees,
    );
    verifier.seed_taint(fn_node);
    let result = verifier.verify_flow(fn_node);

    if result.verified {
        TaintVerification {
            verified: true,
            detail: result.detail,
        }
    } else {
        TaintVerification {
            verified: false,
            detail: result.detail,
        }
    }
}

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

        let snapshots = self.snapshot_files(root, files);
        let (mut symbols, file_ids, snapshot_map) = process_snapshots(&self.auditor, &snapshots)?;
        let file_trees = build_file_trees(&snapshots);

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        // Create DataFlowEngine for cross-file taint verification
        let data_flow = frensense_engine::data_flow::DataFlowEngine::new();

        self.run_taint_analysis(&snapshots, &symbols, &file_trees, &mut all_advisories);
        run_corpus_scan(self, root, &snapshots, &symbols, &data_flow, &file_trees, &mut all_advisories);
        run_findings_modules(root, &snapshots, &symbols, &file_trees, &self.extra_taint_rule_dirs, self.check_deps, &mut all_advisories);
        self.apply_composition(&mut all_advisories);

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
        let symbols = self.auditor.discover_symbols(path, id, content, &language, &tree)?;
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
        self.auditor.discover_events(path, content, &tree, &mut registry)?;

        let opts = AuditOptions {
            file_id: id, path, content, tree: &tree, semantic_ops: &semantic_ops,
            symbols: &registry, graph: registry.graph(), file_trees: &file_trees,
            category_filter: &self.enabled_categories, tag_filter: &self.enabled_tags,
            suite: self.suite, env: self.environment, severity_filter: self.severity_filter,
            ngram_window_size: self.ngram_window_size,
            taint_confidence_interprocedural: self.taint_confidence_interprocedural,
            taint_confidence_intraprocedural: self.taint_confidence_intraprocedural,
            default_taint_max_depth: self.default_taint_max_depth,
        };

        let mut advisories = self.auditor.audit(&opts)?.advisories;
        apply_severity_overrides(&mut advisories, &config.severity_override, &self.severity_overrides);
        self.apply_composition(&mut advisories);
        Ok(advisories)
    }

    /// Applies real composition to advisories, replacing the coincidence counter.
    /// Uses LayerSignals to check if layers are causally related, not just co-located.
    fn apply_composition(&self, advisories: &mut [Advisory]) {
        crate::engine::composition::apply_composition(advisories);
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
        let (mut symbols, file_ids, snapshot_map) = process_snapshots(&self.auditor, &snapshots)?;
        let file_trees = build_file_trees(&snapshots);

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        apply_severity_overrides(&mut all_advisories, &config.severity_override, &self.severity_overrides);
        self.apply_composition(&mut all_advisories);

        #[cfg(feature = "fingerprinting")]
        self.run_profile_analysis(&snapshots, &mut all_advisories);

        self.load_calibration();
        // Create DataFlowEngine for cross-file taint verification
        let data_flow = frensense_engine::data_flow::DataFlowEngine::new();
        run_corpus_scan(self, root, &snapshots, &symbols, &data_flow, &file_trees, &mut all_advisories);
        self.run_taint_analysis(&snapshots, &symbols, &file_trees, &mut all_advisories);
        run_findings_modules(root, &snapshots, &symbols, &file_trees, &self.extra_taint_rule_dirs, self.check_deps, &mut all_advisories);

        // Apply severity overrides to all findings
        apply_severity_overrides(&mut all_advisories, &config.severity_override, &self.severity_overrides);

        if let Some(ref baseline_path) = self.baseline_path {
            if let Ok(prev) = std::fs::read_to_string(baseline_path) {
                if let Ok(fingerprints) = serde_json::from_str::<Vec<String>>(&prev) {
                    let baseline_set: HashSet<String> = fingerprints.into_iter().collect();
                    all_advisories.retain(|a| !baseline_set.contains(&a.fingerprint));
                }
            }
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok((all_advisories, symbols))
    }

    fn run_taint_analysis(
        &self,
        _snapshots: &[super::FileSnapshot],
        _symbols: &SymbolRegistry,
        _file_trees: &std::collections::HashMap<
            String,
            (
                tree_sitter::Tree,
                String,
                Vec<crate::semantics::data_flow::normalization::SemanticOp>,
            ),
        >,
        _all_advisories: &mut Vec<Advisory>,
    ) {
        // Taint analysis removed — detection is now purely corpus-based.
        // The taint analysis engine is retained for cross-file taint verification
        // but is no longer driven by regex rules.
    }

    #[cfg(feature = "fingerprinting")]
    fn run_profile_analysis(&self, snapshots: &[super::FileSnapshot], all_advisories: &mut Vec<Advisory>) {
        let Some(ref profile) = self.profile else { return };

        let mut all_fingerprints = Vec::new();
        for snap in snapshots {
            let mut fps = Vec::new();
            crate::engine::fingerprint::extract_fingerprints(
                snap.tree.root_node(), &snap.content, &snap.path, &mut fps, self.ngram_window_size,
            );
            all_fingerprints.extend(fps);
        }

        for fp in &all_fingerprints {
            let result = profile.style_surprise(fp);
            if result.score > self.profile_threshold {
                all_advisories.push(
                    Advisory::bare("STYLE_ANOMALY", crate::Severity::Warning, FileId(0), std::path::Path::new(&fp.file_path), format!("Style Anomaly: '{}' has {:.0}% unfamiliar patterns.", fp.function_name, result.score * 100.0))
                        .with_confidence(result.score as f32)
                        .with_line(u32::try_from(fp.line).unwrap_or(u32::MAX))
                        .with_content(fp.function_name.clone())
                        .with_enclosing_symbol(fp.function_name.clone())
                        .with_impact("LLM-generated code often violates project conventions — wrong casing, unfamiliar boilerplate, or types never used in this codebase.")
                        .with_improvement("Review the function against project patterns. Consider using established conventions."),
                );
            }
        }

        // Use clustering for near-duplicate detection (replaces pairwise O(n²))
        let clusters = crate::engine::clustering::cluster_functions(&all_fingerprints, 0.75);
        let cluster_advisories = crate::engine::clustering::cluster_to_advisories(&clusters);
        all_advisories.extend(cluster_advisories);

        // Also emit basic info for all clusters (even consistent ones)
        for cluster in &clusters {
            if cluster.members.len() < 2 {
                continue;
            }
            let member_names: Vec<&str> = cluster
                .members
                .iter()
                .map(|m| m.fingerprint.function_name.as_str())
                .collect();
            let first = &cluster.members[0].fingerprint;
            all_advisories.push(
                Advisory::bare(
                    "NEAR_DUPLICATE_FUNCTION",
                    crate::Severity::Info,
                    FileId(0),
                    std::path::Path::new(&first.file_path),
                    format!(
                        "Cluster {}: {} functions are near-duplicates: {}",
                        cluster.id,
                        cluster.members.len(),
                        member_names.join(", ")
                    ),
                )
                .with_confidence(0.8)
                .with_line(u32::try_from(first.line).unwrap_or(u32::MAX))
                .with_content(first.function_name.clone())
                .with_impact("Copy-pasted code diverges over time — one copy may lack security fixes.")
                .with_improvement("Consider extracting shared logic into a common function.")
                .with_tags(["copy-paste", "duplicate", "cluster"]),
            );
        }
    }

    fn snapshot_files(&mut self, _root: &Path, files: &[PathBuf]) -> Vec<FileSnapshot> {
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
            let symbols = match self.auditor.discover_symbols(p, id, &content, &language, &tree) {
                Ok(s) => s,
                Err(e) => {
                    self.file_cache.remove(p);
                    tracing::warn!("symbol discovery failed for {}: {e}", p.display());
                    continue;
                }
            };
            let edges = match self.auditor.scan_for_edges(p, &content, &language, &tree) {
                Ok(e) => e,
                Err(e) => {
                    self.file_cache.remove(p);
                    tracing::warn!("edge discovery failed for {}: {e}", p.display());
                    continue;
                }
            };
            let semantic_ops = self.auditor.extract_semantic_ops(p, &content, &tree);
            self.file_cache.update(p, &content);
            snapshots.push(FileSnapshot { id, path: p.clone(), content, tree, symbols, edges, semantic_ops });
        }
        snapshots
    }

    fn initialize_auditor_and_config(&mut self, root: &Path) -> config::FrensenseConfig {
        let config = config::load_config(root);

        // Wire rules_dir from config to extra taint rule dirs
        if let Some(ref dir) = config.rules_dir {
            let path = root.join(dir);
            if path.is_dir() && !self.extra_taint_rule_dirs.contains(&path) {
                self.extra_taint_rule_dirs.push(path);
            }
        }

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

/// Find a function node by name and line number for semantic filtering.
fn find_function_node<'a>(
    root: tree_sitter::Node<'a>,
    name: &str,
    line: usize,
    source: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut cursor = root.walk();
    let mut best_match: Option<tree_sitter::Node<'a>> = None;
    
    loop {
        let node = cursor.node();
        let kind = node.kind();
        
        if matches!(
            kind,
            "function_item" | "function_declaration" | "method_definition" | "arrow_function"
            | "function" | "formal_parameters"
        ) {
            // Calculate line number for this node
            let node_line = source[..node.start_byte()].chars().filter(|&c| c == '\n').count();
            
            // Skip if too far from target line
            if node_line > line + 5 {
                if cursor.goto_first_child() { continue; }
                loop {
                    if cursor.goto_next_sibling() { break; }
                    if !cursor.goto_parent() { return best_match; }
                }
                continue;
            }
            
            // For named functions, check name match
            if name != "anonymous" {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let node_name = &source[name_node.start_byte()..name_node.end_byte()];
                    if node_name == name && node_line.abs_diff(line) <= 2 {
                        return Some(node);
                    }
                } else if kind == "arrow_function" {
                    if let Some(parent) = node.parent() {
                        if parent.kind() == "variable_declarator" {
                            if let Some(name_node) = parent.child_by_field_name("name") {
                                let node_name = &source[name_node.start_byte()..name_node.end_byte()];
                                if node_name == name && node_line.abs_diff(line) <= 2 {
                                    return Some(node);
                                }
                            }
                        }
                    }
                }
            } else {
                // For anonymous functions, find the closest function at the target line
                // Arrow functions and function expressions are the priority
                if node_line.abs_diff(line) <= 1 {
                    // Prefer arrow functions (more likely to be the anonymous one)
                    if kind == "arrow_function" {
                        // Check if this is the innermost function
                        let has_inner = has_function_child(node);
                        if !has_inner {
                            best_match = Some(node);
                        }
                    } else if best_match.is_none() {
                        best_match = Some(node);
                    }
                }
            }
        }
        
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return best_match;
            }
        }
    }
}

/// Check if a node contains any function child nodes.
fn has_function_child(node: tree_sitter::Node<'_>) -> bool {
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n != node && matches!(
            n.kind(),
            "function_item" | "function_declaration" | "method_definition" | "arrow_function"
            | "function"
        ) {
            return true;
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return false;
            }
        }
    }
}
