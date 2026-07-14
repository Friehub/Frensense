// SPDX-License-Identifier: MIT

use super::Engine;
use super::{FileSnapshot, cache, config};
use crate::engine::auditor::AuditOptions;
use crate::engine::suppression::SuppressConfig;

use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, Result};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

struct ProcessSnapshotsResult<'a> {
    symbols: SymbolRegistry,
    file_ids: Vec<(FileId, PathBuf)>,
    snapshot_map: HashMap<FileId, &'a FileSnapshot>,
}

///
/// # Errors
/// May return an error if the operation fails.
///
/// # Panics
/// May panic if internal assertions fail.
/// Shared snapshot processing: build symbol registry, add edges, discover events.
fn process_snapshots<'a>(
    auditor: &crate::engine::auditor::FrensenseAuditor,
    snapshots: &'a [FileSnapshot],
) -> Result<ProcessSnapshotsResult<'a>> {
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

    Ok(ProcessSnapshotsResult {
        symbols,
        file_ids,
        snapshot_map,
    })
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Build `file_trees` map from snapshots.
fn build_file_trees(
    snapshots: &[FileSnapshot],
) -> HashMap<
    String,
    (
        tree_sitter::Tree,
        String,
        Vec<crate::semantics::data_flow::normalization::SemanticOp>,
    ),
> {
    let mut file_trees = HashMap::new();
    for snap in snapshots {
        file_trees.insert(
            snap.path.to_string_lossy().to_string(),
            (
                snap.tree.clone(),
                snap.content.clone(),
                snap.semantic_ops.clone(),
            ),
        );
    }
    file_trees
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Merge config + CLI severity overrides (CLI wins) into advisories.
fn apply_severity_overrides(
    advisories: &mut [Advisory],
    config_overrides: Option<&HashMap<String, crate::Severity>>,
    cli_overrides: &HashMap<String, crate::Severity>,
) {
    let mut merged = config_overrides.cloned().unwrap_or_default();
    for (rule_id, sev) in cli_overrides {
        merged.insert(rule_id.clone(), *sev);
    }
    for adv in advisories {
        if let Some(sev) = merged.get(&adv.rule_id) {
            adv.severity = *sev;
        }
    }
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Run all findings modules (W1-W7) on snapshots.
fn run_findings_modules(
    root: &Path,
    snapshots: &[FileSnapshot],
    symbols: &SymbolRegistry,
    _file_trees: &HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
    _extra_taint_rule_dirs: &[PathBuf],
    check_deps: bool,
    source_sink: &frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry,
    all_advisories: &mut Vec<Advisory>,
    use_data_flow: bool,
) {
    use crate::engine::findings::{FindingContext, registered_modules};

    let modules = registered_modules();

    // Setup shared state needed by some modules
    let mut dep_resolver = frensense_engine::deps::DependencyResolver::with_check_deps(check_deps);
    dep_resolver.load_project(root);

    // Create a DataFlowEngine for cross-file taint analysis
    let data_flow_engine = frensense_engine::data_flow::DataFlowEngine::new();

    // Instantiate dormant modules
    let alias_tracker = frensense_engine::data_flow::AliasTracker::new();
    let all_symbols = symbols.query_all();
    let mut cross_file_taint =
        frensense_engine::data_flow::cross_file::build_resolver(&all_symbols, symbols.graph());
    let mut exposed_count = 0;

    // Seed the cross-file taint resolver with user input sources
    if use_data_flow {
        for snap in snapshots {
            let root = snap.tree.root_node();
            let mut stack = vec![root];
            while let Some(node) = stack.pop() {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    stack.push(child);
                }

                if node.kind() == "function_declaration"
                    || node.kind() == "arrow_function"
                    || node.kind() == "method_definition"
                    || node.kind() == "function"
                {
                    let mut fn_name_str = String::new();
                    if let Some(name_node) = node.child_by_field_name("name") {
                        if let Ok(name) = name_node.utf8_text(snap.content.as_bytes()) {
                            fn_name_str = name.to_string();
                        }
                    } else if let Some(parent) = node.parent() {
                        // Try to get name from variable declarator: `const myFunc = () => {}`
                        if parent.kind() == "variable_declarator" {
                            if let Some(name_node) = parent.child_by_field_name("name") {
                                if let Ok(name) = name_node.utf8_text(snap.content.as_bytes()) {
                                    fn_name_str = name.to_string();
                                }
                            }
                        } else if parent.kind() == "pair" || parent.kind() == "property_identifier"
                        {
                            if let Some(key_node) = parent.child_by_field_name("key") {
                                if let Ok(name) = key_node.utf8_text(snap.content.as_bytes()) {
                                    fn_name_str = name.to_string();
                                }
                            }
                        }
                    }

                    if fn_name_str.is_empty() {
                        fn_name_str = format!(
                            "anon_{}_{}",
                            node.start_position().row,
                            node.start_position().column
                        );
                    }

                    if let Some(params_node) = node
                        .child_by_field_name("parameters")
                        .or_else(|| node.child_by_field_name("formal_parameters"))
                    {
                        let mut p_cursor = params_node.walk();
                        let mut is_source = false;
                        for param in params_node.children(&mut p_cursor) {
                            if matches!(param.kind(), "(" | ")" | "," | ";" | "self") {
                                continue;
                            }
                            let (param_name, param_type) =
                                frensense_engine::corpus::source_sink::extract_param_info(
                                    param,
                                    &snap.content,
                                );
                            let clean_type = param_type.trim_start_matches(':').trim();

                            if source_sink.is_source_type(clean_type)
                                || param_name.to_lowercase() == "req"
                                || param_name.to_lowercase() == "request"
                                || param_name.to_lowercase() == "event"
                            {
                                is_source = true;
                            }
                        }

                        if is_source {
                            cross_file_taint.register_exposed_taint(
                                &fn_name_str,
                                &snap.path.to_string_lossy(),
                                frensense_engine::data_flow::TaintOrigin::UserInput,
                            );
                            exposed_count += 1;
                        }

                        // Intra-procedural fallback for anonymous functions
                        let mut body_stack = vec![node];
                        let mut is_db_source = false;

                        while let Some(b_node) = body_stack.pop() {
                            let mut b_cursor = b_node.walk();
                            for b_child in b_node.children(&mut b_cursor) {
                                body_stack.push(b_child);
                            }

                            if b_node.kind() == "call_expression"
                                || b_node.kind() == "member_expression"
                            {
                                if let Ok(expr_text) = b_node.utf8_text(snap.content.as_bytes()) {
                                    let expr_lower = expr_text.to_lowercase();
                                    if expr_lower.contains("request.headers")
                                        || expr_lower.contains("req.header")
                                        || expr_lower.contains("req.headers")
                                        || expr_lower == "headers()"
                                        || expr_lower.contains("headers.get")
                                    {
                                        if !is_source {
                                            is_source = true;
                                            cross_file_taint.register_exposed_taint(
                                                &fn_name_str,
                                                &snap.path.to_string_lossy(),
                                                frensense_engine::data_flow::TaintOrigin::UserInput,
                                            );
                                            exposed_count += 1;
                                        }
                                    }
                                }
                            }

                            if b_node.kind() == "call_expression" {
                                if let Some(func_node) = b_node.child_by_field_name("function") {
                                    if let Ok(call_name) =
                                        func_node.utf8_text(snap.content.as_bytes())
                                    {
                                        let lower = call_name.to_lowercase();

                                        let is_safe_base = call_name.starts_with("Object.")
                                            || call_name.starts_with("Array.")
                                            || call_name.starts_with("String.")
                                            || call_name.starts_with("Math.")
                                            || call_name.starts_with("JSON.")
                                            || call_name.starts_with("console.")
                                            || call_name.starts_with("process.");

                                        let mut is_sink = false;

                                        if !is_safe_base {
                                            // Second-order DB taint: mark DB read calls
                                            if lower.ends_with(".findbypk")
                                                || lower.ends_with(".findone")
                                                || lower.ends_with(".findall")
                                                || lower.ends_with(".find")
                                                || lower.ends_with(".query")
                                            {
                                                is_db_source = true;
                                            }

                                            is_sink = lower == "eval"
                                                || lower == "exec"
                                                || lower.ends_with(".query")
                                                || lower == "query"
                                                || lower.ends_with(".send")
                                                || lower == "send"
                                                || lower.ends_with(".json")
                                                || lower == "json"
                                                || lower.ends_with(".sendfile")
                                                || lower == "sendfile"
                                                || lower.ends_with(".sendstatus")
                                                || lower == "sendstatus"
                                                || lower.ends_with(".find")
                                                || lower.ends_with(".findone")
                                                || lower.ends_with(".create")
                                                || lower.ends_with(".insert")
                                                || lower.ends_with(".update")
                                                || lower.ends_with(".remove");
                                        }

                                        if is_source && is_sink {
                                            all_advisories.push(Advisory::bare(
                                            "CROSS_FILE_TAINT",
                                            crate::Severity::Critical,
                                            snap.id,
                                            &snap.path,
                                            format!("User input flows to sensitive sink {} in {}", call_name, fn_name_str),
                                        )
                                        .with_impact("Unvalidated user input reaches a sensitive execution context.")
                                        .with_improvement("Add validation or sanitization to the source before passing data."));
                                        }
                                    }
                                }
                            }
                        }

                        if is_db_source && !is_source {
                            cross_file_taint.register_exposed_taint(
                                &fn_name_str,
                                &snap.path.to_string_lossy(),
                                frensense_engine::data_flow::TaintOrigin::Database,
                            );
                            exposed_count += 1;
                        }
                    }
                }
            }
        }

        eprintln!(
            "DEBUG CROSS_FILE_TAINT: registered {} exposed sources",
            exposed_count
        );
    }

    let mut temporal_analyzer = frensense_engine::temporal::TemporalAnalyzer::new();
    temporal_analyzer.add_default_rules();

    for snap in snapshots {
        let mut ctx = FindingContext {
            symbols,
            dep_resolver: Some(&mut dep_resolver),
            data_flow_engine: Some(&data_flow_engine),
            alias_tracker: Some(&alias_tracker),
            cross_file_taint: Some(&cross_file_taint),
            temporal_analyzer: Some(&temporal_analyzer),
        };
        for module in &modules {
            all_advisories.extend(module.run(snap, &mut ctx));
        }
    }
}

///
/// # Panics
/// May panic if internal assertions fail.
/// Run corpus pattern matching on snapshots.
fn run_corpus_scan(
    engine: &Engine,
    root: &Path,
    snapshots: &[FileSnapshot],
    symbols: &crate::semantics::symbols::SymbolRegistry,
    data_flow: &frensense_engine::data_flow::DataFlowEngine,
    file_trees: &HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
    all_advisories: &mut Vec<Advisory>,
) -> frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry {
    // Load suppressions from .frensense-suppress.yml
    let suppressions = load_suppressions(root);

    let mut corpus_dirs: Vec<&Path> = Vec::new();
    if let Some(ref corpus_dir) = engine.corpus_dir {
        corpus_dirs.push(corpus_dir.as_path());
    }

    let mut registry = frensense_engine::corpus::registry::PatternRegistry::new(
        engine.corpus_threshold,
        engine.ngram_sim_threshold,
    );
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
        return frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry::default();
    }
    let ngram_window_size = engine.ngram_window_size;
    let per_category_calibration = engine.per_category_calibration.clone();
    let calibration = engine.calibration.clone();

    let mut dep_resolver =
        frensense_engine::deps::DependencyResolver::with_check_deps(engine.check_deps);
    dep_resolver.load_project(root);
    let npm_deps = dep_resolver.npm_deps().clone();

    let all_fps: Vec<(
        frensense_engine::fingerprint::FunctionFingerprint,
        tree_sitter::Node<'_>,
        &FileSnapshot,
        frensense_engine::context::FileContext,
    )> = snapshots
        .par_iter()
        .flat_map(|snap| {
            if is_test_file(&snap.path) {
                return Vec::new();
            }
            let start_time = std::time::Instant::now();
            let ctx = frensense_engine::context::FileContext::extract(&snap.path, &snap.content);
            let mut fps = Vec::new();

            eprintln!("Extracting fingerprints for: {}", snap.path.display());

            frensense_engine::fingerprint::extract_fingerprints_with_nodes(
                snap.tree.root_node(),
                &snap.content,
                &snap.path,
                &mut fps,
                ngram_window_size,
            );
            if start_time.elapsed().as_millis() > 500 {
                eprintln!(
                    "Slow fingerprinting for {}: {}ms",
                    snap.path.display(),
                    start_time.elapsed().as_millis()
                );
            }
            fps.into_iter()
                .map(move |(fp, node)| (fp, node, snap, ctx.clone()))
                .collect::<Vec<_>>()
        })
        .collect();

    eprintln!(
        "Fingerprinting completed for {} functions. Beginning scoring pipeline...",
        all_fps.len()
    );
    let scoring_start_time = std::time::Instant::now();

    let new_advisories: Vec<Advisory> = all_fps.into_par_iter().flat_map(|(fp, func_node, snap, actual_context)| {
        let mut local_advisories = Vec::new();

        thread_local! {
            static AST_CACHE: std::cell::RefCell<rustc_hash::FxHashMap<u64, Vec<frensense_engine::corpus::registry::PatternMatch>>> = std::cell::RefCell::new(rustc_hash::FxHashMap::default());
        }
            let mut cache_hasher = rustc_hash::FxHasher::default();
            std::hash::Hash::hash(&fp.ngram_hashes, &mut cache_hasher);
            std::hash::Hash::hash(&fp.structural_markers, &mut cache_hasher);
            std::hash::Hash::hash(&fp.api_calls, &mut cache_hasher);
            std::hash::Hash::hash(&fp.control_flow_hashes, &mut cache_hasher);
            let fp_hash = std::hash::Hasher::finish(&cache_hasher);

            let matches = AST_CACHE.with(|cache| {
                if let Some(cached) = cache.borrow().get(&fp_hash) {
                    return Some(cached.clone());
                }
                None
            });

            let start_time = std::time::Instant::now();
            let use_data_flow = engine.use_data_flow;
            let matches = if let Some(m) = matches {
                m
            } else {
                let m = registry.scan_function(&fp, Some(func_node), Some(&snap.content), Some(&actual_context));
                AST_CACHE.with(|cache| {
                    cache.borrow_mut().insert(fp_hash, m.clone());
                });
                m
            };
            let elapsed = start_time.elapsed().as_millis();
            if elapsed > 500 {
                eprintln!("Slow scoring for {} in {}: {}ms", fp.function_name, snap.path.display(), elapsed);
            }

            if matches.is_empty() {
                return local_advisories;
            }

            for m in matches {
                let impact = m.impact.clone().unwrap_or_else(|| {
                    "Function shape matches a known violation pattern. Unsanitized data from `{{ source }}` reaches the `{{ sink }}` execution context.".to_string()
                });
                let improvement = m.improvement.clone()
                    .unwrap_or_else(|| "Review against corpus example.".to_string());
                let observation = m.observation.clone().unwrap_or_else(|| {
                    format!(
                        "Corpus pattern: {} (score {:.2}) in '{}'",
                        m.pattern_id, m.score, fp.function_name
                    )
                });

                // Apply confidence calibration if available
                // Extract category from pattern ID for per-category calibration
                let category = m.pattern_id.split('_').nth(1).unwrap_or("default");
                let mut confidence = if let Some(ref per_cat_cal) = per_category_calibration
                {
                    per_cat_cal.calibrate(m.score, category)
                } else if let Some(ref params) = calibration {
                    params.calibrate(m.score)
                } else {
                    m.score
                };

                // Verify taint flow if we have a function node
                let mut taint_verified = false;
                let mut taint_detail = String::new();
                let mut source_name = None;
                let mut sink_name = None;
                if use_data_flow {
                    let verification = verify_taint_flow(
                        func_node,
                        &snap.content,
                        &snap.tree,
                        &snap.path,
                        symbols,
                        data_flow,
                        file_trees,
                        registry.source_sink_registry(),
                        &npm_deps,
                    );
                    if verification.verified {
                        taint_verified = true;
                        taint_detail = verification.detail;
                        source_name = verification.source_name;
                        sink_name = verification.sink_name;
                        // Boost confidence for verified findings
                        confidence = (confidence * 1.2).min(0.95);
                    }
                }

                let mut impact = impact;
                let mut improvement = improvement;

                let src_str = source_name.as_deref().unwrap_or("user input");
                let snk_str = sink_name.as_deref().unwrap_or("execution sink");

                impact = impact.replace("{{ source }}", src_str);
                impact = impact.replace("{{ sink }}", snk_str);
                impact = impact.replace("{{source}}", src_str);
                impact = impact.replace("{{sink}}", snk_str);

                improvement = improvement.replace("{{ source }}", src_str);
                improvement = improvement.replace("{{ sink }}", snk_str);
                improvement = improvement.replace("{{source}}", src_str);
                improvement = improvement.replace("{{sink}}", snk_str);

                // T-FIX-1 Composition Logic:
                // If the structural score is below the high-confidence threshold (0.20),
                // we REQUIRE the Taint Engine (Layer 2) to verify the data flow.
                // If it fails to verify, this is a structural false positive and we drop it.
                if !taint_verified && m.score < 0.20 {
                    continue;
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
                    advisory.impact = format!("{impact}\n\nTaint flow verified: {taint_detail}");
                }

                // Check suppressions
                if is_corpus_suppressed(&suppressions, &advisory.rule_id, &snap.path) {
                    continue;
                }

                local_advisories.push(advisory);
            }
        local_advisories
    }).collect();

    eprintln!(
        "Scoring pipeline completed in {}ms.",
        scoring_start_time.elapsed().as_millis()
    );

    all_advisories.extend(new_advisories);

    registry.source_sink_registry().clone()
}

fn load_suppressions(root: &Path) -> Vec<(String, glob::Pattern)> {
    let suppress_file = root.join(".frensense-suppress.yml");
    if !suppress_file.exists() {
        return Vec::new();
    }
    let Ok(content) = std::fs::read_to_string(&suppress_file) else {
        return Vec::new();
    };
    let Ok(config) = serde_yaml::from_str::<crate::engine::suppression::SuppressConfig>(&content)
    else {
        return Vec::new();
    };
    config
        .suppressions
        .into_iter()
        .filter_map(|s| glob::Pattern::new(&s.path).ok().map(|p| (s.rule_id, p)))
        .collect()
}

fn is_corpus_suppressed(
    suppressions: &[(String, glob::Pattern)],
    rule_id: &str,
    path: &std::path::Path,
) -> bool {
    for (sid, pattern) in suppressions {
        if (sid == rule_id || sid == "all") && pattern.matches_path(path) {
            return true;
        }
    }
    false
}

/// Verification result from taint flow analysis.
struct TaintVerification {
    verified: bool,
    detail: String,
    source_name: Option<String>,
    sink_name: Option<String>,
}

/// Verify that taint actually flows from source to sink in a function.
///
/// This uses the `CrossFileVerifier` to check if user-controlled data
/// reaches a dangerous sink, following taint through function calls.
///
/// # Panics
/// May panic if internal assertions fail.
/// Source types and sink names are learned from the corpus.
fn verify_taint_flow(
    fn_node: tree_sitter::Node,
    source: &str,
    tree: &tree_sitter::Tree,
    file_path: &Path,
    symbols: &crate::semantics::symbols::SymbolRegistry,
    data_flow: &frensense_engine::data_flow::DataFlowEngine,
    file_trees: &HashMap<
        String,
        (
            tree_sitter::Tree,
            String,
            Vec<crate::semantics::data_flow::normalization::SemanticOp>,
        ),
    >,
    source_sink: &frensense_engine::corpus::source_sink::CorpusSourceSinkRegistry,
    deps: &std::collections::HashSet<String>,
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
        source_sink,
        deps,
    );
    verifier.seed_taint(fn_node);
    let result = verifier.verify_flow(fn_node);

    if result.verified {
        TaintVerification {
            verified: true,
            detail: result.detail,
            source_name: result.source_name,
            sink_name: result.sink_name,
        }
    } else {
        TaintVerification {
            verified: false,
            detail: result.detail,
            source_name: None,
            sink_name: None,
        }
    }
}

impl Engine {
    /// Runs the project auditor on the given root directory.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read, if configuration fails to load,
    ///
    /// # Panics
    /// May panic if internal assertions fail.
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
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Returns an error if file reading, parsing, or auditing fails.
    pub fn run_files(&mut self, root: &Path, files: &[PathBuf]) -> Result<Vec<Advisory>> {
        let _config = self.initialize_auditor_and_config(root);
        self.file_cache = cache::FileCache::load(root, self.language_filter.as_deref());

        let snapshots = self.snapshot_files(root, files);
        let ProcessSnapshotsResult {
            mut symbols,
            file_ids,
            snapshot_map,
        } = process_snapshots(&self.auditor, &snapshots)?;
        let file_trees = build_file_trees(&snapshots);

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        // Create DataFlowEngine for cross-file taint verification
        let data_flow = frensense_engine::data_flow::DataFlowEngine::new();

        Self::run_taint_analysis(&snapshots, &symbols, &file_trees, &mut all_advisories);
        let source_sink = run_corpus_scan(
            self,
            root,
            &snapshots,
            &symbols,
            &data_flow,
            &file_trees,
            &mut all_advisories,
        );
        run_findings_modules(
            root,
            &snapshots,
            &symbols,
            &file_trees,
            &self.extra_taint_rule_dirs,
            self.check_deps,
            &source_sink,
            &mut all_advisories,
            self.use_data_flow,
        );
        Self::apply_composition(&mut all_advisories);

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok(all_advisories)
    }

    /// Runs the audit on a single virtual file with the given content.
    ///
    /// # Errors
    ///
    /// # Panics
    /// May panic if internal assertions fail.
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

        let mut advisories = self.auditor.audit(&opts)?.advisories;
        apply_severity_overrides(
            &mut advisories,
            config.severity_override.as_ref(),
            &self.severity_overrides,
        );
        Self::apply_composition(&mut advisories);
        Ok(advisories)
    }

    /// Applies real composition to advisories, replacing the coincidence counter.
    ///
    /// # Panics
    /// May panic if internal assertions fail.
    /// Uses `LayerSignals` to check if layers are causally related, not just co-located.
    fn apply_composition(advisories: &mut [Advisory]) {
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
        let ProcessSnapshotsResult {
            mut symbols,
            file_ids,
            snapshot_map,
        } = process_snapshots(&self.auditor, &snapshots)?;
        let file_trees = build_file_trees(&snapshots);

        let mut all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &mut symbols, &file_trees)?;

        apply_severity_overrides(
            &mut all_advisories,
            config.severity_override.as_ref(),
            &self.severity_overrides,
        );
        Self::apply_composition(&mut all_advisories);

        #[cfg(feature = "fingerprinting")]
        self.run_profile_analysis(&snapshots, &mut all_advisories);

        self.load_calibration();
        // Create DataFlowEngine for cross-file taint verification
        let data_flow = frensense_engine::data_flow::DataFlowEngine::new();
        let source_sink = run_corpus_scan(
            self,
            root,
            &snapshots,
            &symbols,
            &data_flow,
            &file_trees,
            &mut all_advisories,
        );
        Self::run_taint_analysis(&snapshots, &symbols, &file_trees, &mut all_advisories);
        run_findings_modules(
            root,
            &snapshots,
            &symbols,
            &file_trees,
            &self.extra_taint_rule_dirs,
            self.check_deps,
            &source_sink,
            &mut all_advisories,
            self.use_data_flow,
        );

        // Apply severity overrides to all findings
        apply_severity_overrides(
            &mut all_advisories,
            config.severity_override.as_ref(),
            &self.severity_overrides,
        );

        if let Some(ref baseline_path) = self.baseline_path
            && let Ok(prev) = std::fs::read_to_string(baseline_path)
            && let Ok(fingerprints) = serde_json::from_str::<Vec<String>>(&prev)
        {
            let baseline_set: HashSet<String> = fingerprints.into_iter().collect();
            all_advisories.retain(|a| !baseline_set.contains(&a.fingerprint));
        }

        self.file_cache.save(root, self.language_filter.as_deref());
        Ok((all_advisories, symbols))
    }

    fn run_taint_analysis(
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
    fn run_profile_analysis(
        &self,
        snapshots: &[super::FileSnapshot],
        all_advisories: &mut Vec<Advisory>,
    ) {
        let Some(ref profile) = self.profile else {
            return;
        };

        let mut all_fingerprints = Vec::new();
        for snap in snapshots {
            let mut fps = Vec::new();
            frensense_engine::fingerprint::extract_fingerprints(
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
                all_advisories.push(
                    Advisory::bare("STYLE_ANOMALY", crate::Severity::Warning, FileId(0), std::path::Path::new(&fp.file_path), format!("Style Anomaly: '{}' has {:.0}% unfamiliar patterns.", fp.function_name, result.score * 100.0))
                        .with_confidence(result.score)
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
                .with_impact(
                    "Copy-pasted code diverges over time — one copy may lack security fixes.",
                )
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
            let symbols = match self
                .auditor
                .discover_symbols(p, id, &content, &language, &tree)
            {
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

///
/// # Panics
/// May panic if internal assertions fail.
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
            "function_item"
                | "function_declaration"
                | "method_definition"
                | "arrow_function"
                | "function"
                | "formal_parameters"
        ) {
            // Calculate line number for this node
            let node_line = source[..node.start_byte()]
                .chars()
                .filter(|&c| c == '\n')
                .count();

            // If we've passed the target line significantly, we can stop searching.
            // Nodes are ordered by start byte/line, so we will never find it.
            if node_line > line + 5 {
                return best_match;
            }

            // For named functions, check name match
            if name == "anonymous" {
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
            } else if let Some(name_node) = node.child_by_field_name("name") {
                let node_name = &source[name_node.start_byte()..name_node.end_byte()];
                if node_name == name && node_line.abs_diff(line) <= 2 {
                    return Some(node);
                }
            } else if kind == "arrow_function"
                && let Some(parent) = node.parent()
                && parent.kind() == "variable_declarator"
                && let Some(name_node) = parent.child_by_field_name("name")
            {
                let node_name = &source[name_node.start_byte()..name_node.end_byte()];
                if node_name == name && node_line.abs_diff(line) <= 2 {
                    return Some(node);
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

///
/// # Panics
/// May panic if internal assertions fail.
/// Check if a node contains any function child nodes.
fn has_function_child(node: tree_sitter::Node<'_>) -> bool {
    let mut cursor = node.walk();
    loop {
        let n = cursor.node();
        if n != node
            && matches!(
                n.kind(),
                "function_item"
                    | "function_declaration"
                    | "method_definition"
                    | "arrow_function"
                    | "function"
            )
        {
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

fn is_test_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let stem = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

    // Check filename patterns
    if name.ends_with(".test.ts")
        || name.ends_with(".test.tsx")
        || name.ends_with(".test.js")
        || name.ends_with(".test.jsx")
        || name.ends_with(".spec.ts")
        || name.ends_with(".spec.tsx")
        || name.ends_with(".spec.js")
        || name.ends_with(".spec.jsx")
        || name.ends_with("_test.rs")
        || name.ends_with(".test.rs")
    {
        return true;
    }

    // Check if in test directories
    let path_str = path.to_string_lossy();
    if path_str.contains("/tests/")
        || path_str.contains("/test/")
        || path_str.contains("__tests__/")
        || path_str.contains("/__mocks__/")
        || path_str.contains("/mocks/")
    {
        return true;
    }

    // Check for mock files
    if stem.starts_with("mock") || stem.to_lowercase().ends_with(".mock") {
        return true;
    }

    false
}
