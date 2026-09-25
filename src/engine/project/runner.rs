// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! The scan runner: collect files → `data_flow::scan` → advisories.
//!
//! This is the whole consumer pipeline. Advisory text is generated from the
//! flow itself (source access path, sink name, exact sink line/column),
//! never from corpus pattern text or templates.

use super::Engine;
use crate::engine::files::collect_files;
use crate::{Advisory, Result, Severity};
use std::path::Path;

use frensense_engine::analysis::taint::config::TaintConfig;
use frensense_engine::analysis::taint::engine::BackwardVerdict;
use frensense_engine::analysis::taint::facts::{
    FactTable, config_from_spec, fact_table_from_entries, fact_table_from_spec,
};
use frensense_engine::scan;
use frensense_engine::scan::LocatedFinding;

impl Engine {
    /// Scan a directory tree or a single file. Returns advisories.
    ///
    /// # Errors
    /// Returns an error if the path does not exist or a bundle cannot be
    /// loaded.
    pub fn run(&mut self, root: &Path) -> Result<Vec<Advisory>> {
        let paths = collect_files(root, self.language_filter.as_ref());

        // Read all scannable sources.
        let mut files: Vec<(String, String, String)> = Vec::new();
        let mut id_by_path: rustc_hash::FxHashMap<String, usize> = rustc_hash::FxHashMap::default();
        for (seq, p) in paths.iter().enumerate() {
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .map(str::to_lowercase)
                .unwrap_or_default();
            if frensense_lang::spec_for_ext(&ext).is_none() {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(p) else {
                continue;
            };
            id_by_path.insert(p.to_string_lossy().into_owned(), seq);
            files.push((p.to_string_lossy().into_owned(), content, ext));
        }

        // Built-in config + fact table from the language specs present.
        let (config, mut facts) = build_spec_tables(&files);

        // Merge corpus-owned seed facts (deployment-specific knowledge:
        // session-store roots, project conventions). Spec tables first,
        // seed facts over them, bundle facts last (most specific wins).
        let seed_paths = [
            self.seed_facts_path.clone(),
            Some(root.join("frensense-seed-facts.json")),
            std::env::current_dir()
                .ok()
                .map(|d| d.join("corpus/facts/seed_facts.json")),
        ];
        for p in seed_paths.into_iter().flatten() {
            if let Err(e) =
                frensense_engine::analysis::taint::facts::seed::SeedFacts::load_and_apply(
                    &p, &mut facts,
                )
            {
                tracing::warn!("{e}");
            }
        }

        // Merge learned facts from the .frc bundle, if any.
        if let Some(bundle_bytes) = self.load_bundle_bytes(root)? {
            match frensense_bundler::format::load_bundle(bundle_bytes) {
                Ok(loaded) => {
                    facts.merge(&fact_table_from_entries(&loaded.learned_facts));
                }
                Err(e) => {
                    tracing::warn!("failed to load corpus bundle: {e}");
                }
            }
        }

        let result = scan::scan(&files, &config, &facts);
        for err in &result.errors {
            tracing::warn!("scan error: {err}");
        }

        let mut advisories = Vec::new();
        // Non-dataflow policy findings (weak crypto, insecure config):
        // these are assertions about API/constant choices, no taint path
        // exists, so they bypass the flow machinery entirely.
        for lc in &result.located_checker {
            let (c, path, line, col) = (&lc.finding, &lc.file, lc.line, lc.column);
            let Some(&seq) = id_by_path.get(path) else {
                continue;
            };
            advisories.push(
                advisory_from_checker(
                    c,
                    path,
                    line,
                    col,
                    crate::engine::project::Engine::next_file_id(seq),
                )
                .with_end_line(lc.end_line),
            );
        }
        // Dedup gate, two layers:
        //
        // 1. Exact gate, the same (file, line, sink, source) emitted for
        //    several sink arguments collapses to one finding.
        // 2. Shape gate, SSA variants of one flow (same path shape: same
        //    functions, same hops, same field names, different SSA numbering)
        //    collapse to the variant with the most steps (the richest
        //    path). Grouping includes the sink location so the same helper
        //    called from two different sites still reports both.
        let mut exact: std::collections::HashSet<(String, u32, String, String)> =
            std::collections::HashSet::new();
        let mut candidates: Vec<&_> = Vec::new();
        for loc in &result.located {
            let f = &loc.finding;
            if f.verdict != BackwardVerdict::Vulnerable {
                continue;
            }
            // Sink-signature gate: the engine already evaluated per-slot
            // danger (jwt.verify(token, secret) slot 1 = secret,
            // query(sql, params) slot 1+ = binding channel). A `Vulnerable`
            // verdict on a non-dangerous slot must not become an advisory,
            // `alert` is None exactly when the slot is a safe channel.
            if f.alert.is_none() {
                continue;
            }
            let key = (
                loc.file.clone(),
                loc.line,
                f.sink.clone(),
                f.source_desc.clone().unwrap_or_default(),
            );
            if !exact.insert(key) {
                continue;
            }
            candidates.push(loc);
        }
        // Shape gate: keep the richest path per (shape, sink location).
        let kept = dedup_keep_richest(&candidates);
        for i in kept {
            let loc = candidates[i];
            let Some(&seq) = id_by_path.get(&loc.file) else {
                continue;
            };
            advisories.push(advisory_from_finding(
                &loc.finding,
                &loc.file,
                loc.line,
                loc.column,
                crate::engine::project::Engine::next_file_id(seq),
            ));
        }

        advisories.retain(|a| a.confidence >= self.min_confidence);
        if let Some(sev) = self.severity_filter {
            advisories.retain(|a| a.severity.meets_threshold(sev));
        }
        Ok(advisories)
    }

    fn load_bundle_bytes(&self, root: &Path) -> Result<Option<&'static [u8]>> {
        if let Some(static_bytes) = self.corpus_bundle {
            return Ok(Some(static_bytes));
        }
        if let Some(ref path) = self.corpus_bundle_path {
            let bytes = std::fs::read(path).map_err(crate::FrensenseError::Io)?;
            let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
            return Ok(Some(leaked));
        }
        let default_path = root.join("frensense-corpus.frc");
        if default_path.exists() {
            let bytes = std::fs::read(&default_path).map_err(crate::FrensenseError::Io)?;
            let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
            return Ok(Some(leaked));
        }
        Ok(None)
    }
}

/// Build the merged `TaintConfig` and `FactTable` from the language specs of
/// the files being scanned.
fn build_spec_tables(files: &[(String, String, String)]) -> (TaintConfig, FactTable) {
    let mut config = TaintConfig::default();
    let mut facts = FactTable::default();
    let mut seen: std::collections::BTreeSet<&'static str> = std::collections::BTreeSet::new();
    for (_, _, ext) in files {
        if let Some(spec) = frensense_lang::spec_for_ext(ext)
            && seen.insert(spec.name())
        {
            let c = config_from_spec(spec);
            config.sources.extend(c.sources);
            config.sinks.extend(c.sinks);
            config.sanitizers.extend(c.sanitizers);
            facts.merge(&fact_table_from_spec(spec));
        }
    }
    (config, facts)
}

/// Build the compiler-style advisory for one non-dataflow policy finding.
fn advisory_from_checker(
    c: &frensense_engine::checks::CheckerFinding,
    file: &str,
    line: u32,
    column: u32,
    file_id: crate::FileId,
) -> Advisory {
    let path = Path::new(file);
    let title = format!("Policy violation: {} ({})", c.rule, c.function);
    let mut advisory = Advisory::bare(title, Severity::Warning, file_id, path, c.message.clone())
        .with_confidence(1.0)
        .with_line(line)
        .with_column(column)
        .with_content(c.function.clone())
        .with_enclosing_symbol(c.function.clone())
        .with_impact(format!(
            "{} at {}:{}, insecure cryptographic/configuration choice.",
            c.rule, file, line
        ))
        .with_improvement(format!(
            "Replace the weak primitive in `{}` with a modern alternative \
             (bcrypt/argon2 for passwords, SHA-256+ for digests).",
            c.function
        ))
        .with_tags(["checker", &c.rule]);
    advisory.requires_human = false;
    advisory.fingerprint = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        file.hash(&mut h);
        c.rule.hash(&mut h);
        c.function.hash(&mut h);
        line.hash(&mut h);
        format!("{:016x}", h.finish())
    };
    advisory
}

/// Build the compiler-style advisory for one vulnerable flow.
fn advisory_from_finding(
    f: &frensense_engine::analysis::taint::engine::SinkFinding,
    file: &str,
    line: u32,
    column: u32,
    file_id: crate::FileId,
) -> Advisory {
    let src = f.source_desc.as_deref().unwrap_or("user input");
    let path = Path::new(file);
    // Rank by finding class: IDOR-shaped flows (access-control queries) are
    // Warning; raw injection channels stay Critical.
    // Role-first ranking: what the sink DOES with the data decides the
    // default level; the shape class (Idor) can only lower it further.
    let role_level = match f.role.default_level() {
        "critical" => Severity::Critical,
        "warning" => Severity::Warning,
        _ => Severity::Info,
    };
    let class_tag = f.role.tag();
    let (severity, title) = match (f.finding_class, role_level) {
        // Access-control query payloads: ranked by role, retitled.
        (frensense_engine::analysis::taint::engine::FindingClass::Idor, Severity::Critical) => (
            Severity::Warning,
            format!(
                "User-controlled query field in `{}` (access-control review)",
                f.sink
            ),
        ),
        // Response/validation sinks: review-level regardless of shape.
        (_, Severity::Info) => (
            Severity::Info,
            format!(
                "User data reaches `{}` {} (review: role={})",
                f.sink,
                match f.finding_class {
                    frensense_engine::analysis::taint::engine::FindingClass::Idor =>
                        "as a query field",
                    frensense_engine::analysis::taint::engine::FindingClass::Injection =>
                        "as a value",
                },
                f.role.tag()
            ),
        ),
        (_, level) => (
            level,
            format!("Unsanitized data from `{src}` reaches sink `{}`", f.sink),
        ),
    };
    let flow = f.path.render_text();
    let observation = format!(
        "Tainted value from `{src}` flows to sink `{}` (argument {}) in function `{}`.\nTaint path:\n{}",
        f.sink,
        f.arg_slot,
        f.function,
        flow.trim_end()
    );
    let _ = class_tag;
    let mut advisory = Advisory::bare(title, severity, file_id, path, observation)
        .with_confidence(1.0)
        .with_line(line)
        .with_column(column)
        .with_content(f.function.clone())
        .with_enclosing_symbol(f.function.clone())
        .with_impact(format!(
            "Unsanitized data from `{src}` reaches the `{}` sink at {}:{}.",
            f.sink, file, line
        ))
        .with_improvement(format!(
            "Sanitize `{src}` before it reaches `{}` in `{}`.",
            f.sink, f.function
        ))
        .with_tags(["taint", class_tag]);
    advisory.requires_human = false;
    // Taint path steps (source→sink) for SARIF codeFlows / rich clients.
    advisory.taint_steps = f
        .path
        .steps
        .iter()
        .zip(&f.path.spans)
        .map(|(step, span)| {
            (
                step.describe(),
                span.as_ref()
                    .map(|(file, (start, _))| (file.clone(), *start)),
            )
        })
        .collect();
    advisory.fingerprint = {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        file.hash(&mut h);
        f.sink.hash(&mut h);
        f.function.hash(&mut h);
        src.hash(&mut h);
        format!("{:016x}", h.finish())
    };
    advisory
}

/// Shape-based dedup over located findings: group by (path shape, sink file,
/// sink line) and keep one representative per group, the finding with the
/// most path steps (the richest path). Returns indices into `candidates`.
///
/// The engine's per-root BFS can emit several SSA variants of one flow at a
/// single sink call (different roots reaching the same argument through
/// equivalent hops). They are the same bug: same functions, same field
/// accesses, same call structure, only SSA numbering differs, which
/// `TaintPath::shape_id` ignores. Printing each variant wastes triage time
/// and buries distinct findings under noise.
fn dedup_keep_richest(candidates: &[&LocatedFinding]) -> Vec<usize> {
    // Number of steps whose span resolved to a real file position, the
    // "richest" representative is the one a consumer can annotate most.
    fn resolved(loc: &LocatedFinding) -> usize {
        loc.finding
            .path
            .spans
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
    let mut best_by_shape: std::collections::HashMap<(String, String, u32), usize> =
        std::collections::HashMap::new();
    for (i, loc) in candidates.iter().enumerate() {
        let key = (loc.finding.path.shape_id(), loc.file.clone(), loc.line);
        match best_by_shape.get(&key).copied() {
            Some(j) if resolved(candidates[j]) >= resolved(loc) => {}
            _ => {
                best_by_shape.insert(key, i);
            }
        }
    }
    let mut kept: Vec<usize> = best_by_shape.into_values().collect();
    kept.sort_unstable();
    kept
}

#[cfg(test)]
mod shape_dedup_tests {
    use super::dedup_keep_richest;
    use frensense_engine::analysis::taint::engine::{BackwardVerdict, FindingClass, SinkFinding};
    use frensense_engine::analysis::taint::path::{PathStep, TaintPath};
    use frensense_engine::analysis::taint::role::SinkRole;
    use frensense_engine::scan::LocatedFinding;

    /// Build a finding whose path is `steps` assignments; the first
    /// `resolved` of them carry spans. Identical step sequence + tag
    /// produces an identical shape_id (variable numbers are ignored).
    fn located(file: &str, line: u32, steps: usize, resolved: usize, tag: &str) -> LocatedFinding {
        let finding = SinkFinding {
            function: "handler".to_string(),
            sink: "exec".to_string(),
            arg_slot: 0,
            alert: Some("test".to_string()),
            verdict: BackwardVerdict::Vulnerable,
            finding_class: FindingClass::Injection,
            role: SinkRole::Execution,
            source_desc: Some(format!("source-{tag}")),
            sink_span: None,
            path: TaintPath {
                steps: (0..steps)
                    .map(|i| PathStep::Assignment {
                        function: format!("f{tag}"),
                        variable: i as u32,
                    })
                    .collect(),
                spans: (0..steps)
                    .map(|i| {
                        if i < resolved {
                            Some((file.to_string(), (0, 1)))
                        } else {
                            None
                        }
                    })
                    .collect(),
            },
        };
        LocatedFinding {
            finding,
            file: file.to_string(),
            line,
            column: 1,
        }
    }

    #[test]
    fn same_shape_same_location_keeps_richest() {
        let a = located("t.ts", 3, 5, 0, "a");
        let b = located("t.ts", 3, 5, 5, "a");
        let kept = dedup_keep_richest(&[&a, &b]);
        assert_eq!(kept, vec![1], "the fully-spanned variant must win");
    }

    #[test]
    fn same_shape_different_lines_keeps_both() {
        let a = located("t.ts", 3, 5, 5, "a");
        let b = located("t.ts", 9, 5, 5, "a");
        let kept = dedup_keep_richest(&[&a, &b]);
        assert_eq!(kept.len(), 2, "distinct sink sites must both report");
    }

    #[test]
    fn same_shape_different_files_keeps_both() {
        let a = located("a.ts", 3, 5, 5, "a");
        let b = located("b.ts", 3, 5, 5, "a");
        let kept = dedup_keep_richest(&[&a, &b]);
        assert_eq!(kept.len(), 2, "cross-file same-shape flows are distinct");
    }

    #[test]
    fn first_wins_on_equal_richness() {
        let a = located("t.ts", 3, 5, 2, "a");
        let b = located("t.ts", 3, 5, 2, "a");
        let kept = dedup_keep_richest(&[&a, &b]);
        assert_eq!(kept, vec![0], "stable: the earlier candidate survives");
    }

    #[test]
    fn different_step_sequences_are_different_shapes() {
        let a = located("t.ts", 3, 3, 3, "a");
        let b = located("t.ts", 3, 5, 5, "a");
        let kept = dedup_keep_richest(&[&a, &b]);
        assert_eq!(kept.len(), 2, "longer walk = different flow = both report");
    }
}
