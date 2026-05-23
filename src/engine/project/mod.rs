// SPDX-License-Identifier: MIT

pub mod cache;
pub mod config;
pub mod helpers;

use crate::engine::auditor::{AuditOptions, GenSenseAuditor, ScanResult};
use crate::engine::suppression::SuppressConfig;
use crate::parser::ParserRegistry;
use crate::semantics::symbols::SymbolRegistry;
use crate::{Advisory, FileId, GenSenseEnvironment, ProjectRule, Result, SourceRegistry};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Detailed internal snapshot of a file's analysis artifacts.
pub struct FileSnapshot {
    pub id: FileId,
    pub path: PathBuf,
    pub content: String,
    pub tree: tree_sitter::Tree,
    pub symbols: Vec<crate::semantics::symbols::Symbol>,
    pub edges: Vec<(String, String)>,
    pub semantic_ops: Vec<crate::semantics::data_flow::normalization::SemanticOp>,
}

pub struct Engine {
    auditor: GenSenseAuditor,
    project_rules: Vec<Box<dyn ProjectRule>>,
    source_registry: SourceRegistry,
    min_confidence: f32,
    environment: GenSenseEnvironment,
    enabled_categories: HashSet<String>,
    enabled_tags: HashSet<String>,
    extra_rule_dirs: Vec<PathBuf>,
    no_builtin_rules: bool,
    isolate_rules: bool,
    language_filter: Option<Vec<&'static str>>,
    file_cache: cache::FileCache,
    cache_root: Option<PathBuf>,
}

impl Engine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            auditor: GenSenseAuditor::new(Vec::new()),
            project_rules: Vec::new(),
            source_registry: SourceRegistry::new(),
            min_confidence: 0.1,
            environment: GenSenseEnvironment::Development,
            enabled_categories: HashSet::new(),
            enabled_tags: HashSet::new(),
            extra_rule_dirs: Vec::new(),
            no_builtin_rules: false,
            isolate_rules: false,
            language_filter: None,
            file_cache: cache::FileCache::default(),
            cache_root: None,
        }
    }

    pub const fn set_min_confidence(&mut self, val: f32) {
        self.min_confidence = val;
    }

    pub const fn set_environment(&mut self, env: GenSenseEnvironment) {
        self.environment = env;
    }

    pub fn set_rules(&mut self, rules: Vec<Box<dyn crate::GenSenseRule>>) {
        self.auditor.set_rules(rules);
    }

    #[must_use]
    pub const fn auditor(&self) -> &crate::GenSenseAuditor {
        &self.auditor
    }

    #[must_use]
    pub fn project_rules(&self) -> &[Box<dyn crate::ProjectRule>] {
        &self.project_rules
    }

    #[must_use]
    pub fn list_rules(&self) -> Vec<(String, String, String)> {
        let mut rules = Vec::new();
        for r in self.auditor.rules() {
            let meta = r.metadata();
            rules.push((
                meta.id.to_string(),
                meta.name.to_string(),
                format!("{:?}", meta.severity),
            ));
        }
        for r in &self.project_rules {
            let meta = r.metadata();
            rules.push((
                meta.id.to_string(),
                meta.name.to_string(),
                format!("{:?}", meta.severity),
            ));
        }
        rules
    }

    pub fn enable_tag(&mut self, tag: &str) {
        self.enabled_tags.insert(tag.to_string());
    }

    pub fn enable_category(&mut self, category: &str) {
        self.enabled_categories.insert(category.to_string());
    }

    pub fn add_rule_dir<P: Into<PathBuf>>(&mut self, path: P) {
        self.extra_rule_dirs.push(path.into());
    }

    pub const fn set_no_builtin_rules(&mut self, val: bool) {
        self.no_builtin_rules = val;
    }

    pub const fn set_isolate_rules(&mut self, val: bool) {
        self.isolate_rules = val;
    }

    pub fn set_language_filter(&mut self, extensions: &[&'static str]) {
        self.language_filter = Some(extensions.to_vec());
    }

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
        self.file_cache = cache::FileCache::load(root);

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

        let all_advisories =
            self.perform_parallel_audit(&file_ids, &snapshot_map, &symbols, &file_trees)?;

        self.file_cache.save(root);
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

        let opts = AuditOptions {
            file_id: id,
            path,
            content,
            tree: &tree,
            semantic_ops: &semantic_ops,
            symbols: &registry,
            file_trees: &file_trees,
            category_filter: &self.enabled_categories,
            tag_filter: &self.enabled_tags,
            env: self.environment,
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

        if let Some(overrides) = &config.severity_override {
            for adv in &mut advisories {
                if let Some(sev) = overrides.get(&adv.rule_id) {
                    adv.severity = *sev;
                }
            }
        }

        Ok(advisories)
    }

    /// Runs a detailed audit, returning both advisories and the assembled symbol registry.
    ///
    /// # Errors
    /// Returns an error if file reading or parsing fails.
    pub fn run_detailed(&mut self, root: &Path) -> Result<(Vec<Advisory>, SymbolRegistry)> {
        if !root.exists() {
            return Err(crate::GenSenseError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("path does not exist: {}", root.display()),
            )));
        }
        self.file_cache = cache::FileCache::load(root);
        self.cache_root = Some(root.to_path_buf());

        let config = self.initialize_auditor_and_config(root);
        let snapshots = self.collect_and_snapshot_files(root)?;

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
            self.perform_parallel_audit(&file_ids, &snapshot_map, &symbols, &file_trees)?;

        for rule in &self.project_rules {
            let project_advisories = rule.check_project(&symbols, &self.source_registry);
            for a in project_advisories {
                if a.confidence >= self.min_confidence {
                    all_advisories.push(a);
                }
            }
        }

        if let Some(overrides) = &config.severity_override {
            for adv in &mut all_advisories {
                if let Some(sev) = overrides.get(&adv.rule_id) {
                    adv.severity = *sev;
                }
            }
        }

        self.file_cache.save(root);
        Ok((all_advisories, symbols))
    }

    fn initialize_auditor_and_config(&mut self, root: &Path) -> config::GenSenseConfig {
        let config = config::load_config(root);

        if !self.isolate_rules {
            let mut dirs = self.extra_rule_dirs.clone();
            if let Some(config_rules_dir) = &config.rules_dir {
                dirs.push(PathBuf::from(config_rules_dir));
            }
            let (rules, project_rules) =
                GenSenseAuditor::build_rule_set(root, &dirs, self.no_builtin_rules);
            self.auditor.set_rules(rules);
            self.project_rules = project_rules;

            if let Some(disabled) = &config.disabled_rules {
                let disabled_set: HashSet<&str> =
                    disabled.iter().map(std::string::String::as_str).collect();
                self.auditor
                    .retain_rules(|r| !disabled_set.contains(r.id()));
                self.project_rules
                    .retain(|r| !disabled_set.contains(r.metadata().id.as_ref()));
            }
        } else if !self.extra_rule_dirs.is_empty() {
            let (user_rules, user_project_rules) =
                crate::engine::auditor::user_rules::load_user_rules(root, &self.extra_rule_dirs);
            self.auditor.add_rules(user_rules);
            self.project_rules.extend(user_project_rules);
        }

        let suppress_file = root.join(".gensense-suppress.yml");
        if suppress_file.exists()
            && let Ok(content) = std::fs::read_to_string(suppress_file)
            && let Ok(supp_config) = serde_yaml::from_str::<SuppressConfig>(&content)
        {
            self.auditor.set_suppressions(supp_config);
        }
        config
    }

    fn collect_and_snapshot_files(&mut self, root: &Path) -> Result<Vec<FileSnapshot>> {
        let files = Self::collect_files(root, self.language_filter.as_ref());
        let mut snapshots = Vec::new();
        for p in files {
            let content = match std::fs::read_to_string(&p) {
                Ok(c) => c,
                Err(e) => {
                    self.file_cache.remove(&p);
                    return Err(crate::GenSenseError::Io(e));
                }
            };
            if self.file_cache.is_unchanged(&p, &content) {
                continue;
            }
            let id = self.source_registry.register(&p, content.clone());
            let auditor = &self.auditor;
            match auditor.parse_source(&p, &content) {
                Ok((language, tree)) => {
                    let symbols = auditor.discover_symbols(&p, id, &content, &language, &tree);
                    let edges = auditor.scan_for_edges(&p, &content, &language, &tree);
                    let semantic_ops = auditor.extract_semantic_ops(&p, &content, &tree);
                    if let (Ok(symbols), Ok(edges)) = (symbols, edges) {
                        self.file_cache.update(&p, &content);
                        snapshots.push(FileSnapshot {
                            id,
                            path: p,
                            content,
                            tree,
                            symbols,
                            edges,
                            semantic_ops,
                        });
                    } else {
                        self.file_cache.remove(&p);
                        return Err(crate::GenSenseError::Io(std::io::Error::other(
                            "symbol or edge discovery failed",
                        )));
                    }
                }
                Err(e) => {
                    self.file_cache.remove(&p);
                    return Err(e);
                }
            }
        }
        Ok(snapshots)
    }

    fn perform_parallel_audit(
        &self,
        file_ids: &[(FileId, PathBuf)],
        snapshot_map: &HashMap<FileId, &FileSnapshot>,
        symbols: &SymbolRegistry,
        file_trees: &HashMap<
            String,
            (
                tree_sitter::Tree,
                String,
                Vec<crate::semantics::data_flow::normalization::SemanticOp>,
            ),
        >,
    ) -> Result<Vec<Advisory>> {
        let results: Result<Vec<ScanResult>> = file_ids
            .iter()
            .map(|(id, p)| {
                let snap = snapshot_map.get(id).ok_or_else(|| {
                    crate::GenSenseError::Engine(format!(
                        "Missing snapshot for file ID {} at path {}",
                        id.0,
                        p.display()
                    ))
                })?;
                let opts = AuditOptions {
                    file_id: *id,
                    path: p,
                    content: &snap.content,
                    tree: &snap.tree,
                    semantic_ops: &snap.semantic_ops,
                    symbols,
                    file_trees,
                    category_filter: &self.enabled_categories,
                    tag_filter: &self.enabled_tags,
                    env: self.environment,
                };
                let result = self.auditor.audit(&opts)?;

                let advisories = result.advisories;

                Ok(ScanResult {
                    advisories,
                    #[cfg(feature = "fingerprinting")]
                    fingerprints: result.fingerprints,
                })
            })
            .collect();

        let mut all_advisories = Vec::new();
        for result in results? {
            for a in result.advisories {
                if a.confidence >= self.min_confidence {
                    all_advisories.push(a);
                }
            }
        }
        Ok(all_advisories)
    }

    fn collect_files(root: &Path, language_filter: Option<&Vec<&'static str>>) -> Vec<PathBuf> {
        if root.is_file() {
            return vec![root.to_path_buf()];
        }
        WalkDir::new(root)
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
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .filter(|p| ParserRegistry::is_supported(p))
            .filter(|p| {
                if let Some(allowed) = language_filter {
                    let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("");
                    allowed.contains(&ext)
                } else {
                    true
                }
            })
            .collect()
    }
}

pub struct ProjectAuditor {
    rules: Vec<Box<dyn ProjectRule>>,
}

impl ProjectAuditor {
    #[must_use]
    pub fn new(rules: Vec<Box<dyn ProjectRule>>) -> Self {
        Self { rules }
    }

    #[must_use]
    pub fn run(
        &self,
        symbols: &SymbolRegistry,
        sources: &SourceRegistry,
        _env: GenSenseEnvironment,
    ) -> Vec<Advisory> {
        let mut advisories = Vec::new();
        for rule in &self.rules {
            advisories.extend(rule.check_project(symbols, sources));
        }
        advisories
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
