// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Task 4.5: Corpus Graph Store
//!
//! Decouples the **representation phase** (parse → lower → SSA → SVFG build →
//! interprocedural linking, expensive, stable) from the **query phase** (taint
//! traversal under a given source/sink/sanitizer rule set, fast, repeated).
//!
//! The pipeline mirrors Joern's CPG-database model:
//!
//! ```text
//! Ingestion (run once per repo, cached):
//!   source files → FunctionIRs → ProgramSvfg → ProgramSvfgParts → disk
//!
//! Query (run per rule set, fast):
//!   load ProgramSvfgParts → ProgramSvfg::from_parts(parts, irs, NEW config)
//!   → forward / demand-driven / context-sensitive traversal → alerts
//! ```
//!
//! The stored payload is *config-independent structure*: per-function SVFGs,
//! call bindings, cross edges and topological order. Summaries and
//! pass-through suppression depend on the rule set and are recomputed at load
//! (see [`ProgramSvfg::from_parts`]), which is exactly why a new sink
//! configuration can re-scan a cached repository without re-running the
//! representation pipeline.
//!
//! The envelope carries a format version and a blake3 content hash so stale
//! caches (IR edited since the graph was built) and corrupted files are
//! detected instead of silently producing wrong alerts.

use std::path::Path;

use rustc_hash::FxHashMap;

use crate::analysis::forward::{ProgramSvfg, ProgramSvfgParts};
use crate::analysis::taint::config::TaintConfig;
use crate::ir::function::FunctionIR;

/// Bump on any breaking change to `ProgramSvfgParts`' on-disk layout.
/// Loaders reject files whose version differs (no silent misparse).
pub const STORE_FORMAT_VERSION: u32 = 1;

/// Versioned, hashed envelope around the serialised program structure.
#[cfg(feature = "serialize")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SvfgStore {
    /// On-disk layout version (see [`STORE_FORMAT_VERSION`]).
    pub format_version: u32,
    /// blake3 hash of the serialised payload, detects corruption/staleness.
    pub content_hash: String,
    /// The config-independent structure (see `ProgramSvfg::to_parts`).
    pub parts: ProgramSvfgParts,
}

#[cfg(feature = "serialize")]
impl SvfgStore {
    /// Build a store envelope from a program graph.
    pub fn from_program(prog: &ProgramSvfg) -> Self {
        let parts = prog.to_parts();
        let content_hash = hash_parts(&parts);
        Self {
            format_version: STORE_FORMAT_VERSION,
            content_hash,
            parts,
        }
    }

    /// Serialise to pretty JSON and write to `path` (creating parents).
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)
    }

    /// Read + parse + version-check + hash-check a store from disk.
    ///
    /// Fails with `InvalidData` if the format version differs or the payload
    /// hash no longer matches the content (corruption / partial write).
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let store: Self = serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        if store.format_version != STORE_FORMAT_VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "svfg store format mismatch: file has v{}, loader expects v{}",
                    store.format_version, STORE_FORMAT_VERSION
                ),
            ));
        }
        let actual = hash_parts(&store.parts);
        if actual != store.content_hash {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "svfg store content hash mismatch (corrupted or stale cache)",
            ));
        }
        Ok(store)
    }

    /// Reconstruct a queryable [`ProgramSvfg`] under `config` using the
    /// caller-supplied IRs (the live IR is authoritative for predicates like
    /// `is_source`).
    ///
    /// `irs` may omit functions: they fall back to the store's own IR copies,
    /// making the loaded graph self-contained when the original IRs are gone.
    pub fn into_program<'a>(
        self,
        irs: &FxHashMap<String, &'a FunctionIR>,
        config: &TaintConfig,
    ) -> ProgramSvfg<'a> {
        ProgramSvfg::from_parts(self.parts, irs, config)
    }
}

/// Deterministic content hash over the parts payload (blake3, hex).
#[cfg(feature = "serialize")]
fn hash_parts(parts: &ProgramSvfgParts) -> String {
    let bytes = serde_json::to_vec(parts).expect("serialising ProgramSvfgParts cannot fail");
    blake3::hash(&bytes).to_hex().to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, feature = "serialize"))]
mod tests {
    use super::*;
    use crate::analysis::forward::InterproceduralTaintEngine;
    use crate::ir::function::*;

    fn meta(name: &str) -> VarMetadata {
        VarMetadata {
            source_name: Some(name.to_string()),
            type_name: None,
            byte_range: None,
            is_memory_state: false,
            object_keys: Vec::new(),
        }
    }

    fn mem_meta(name: &str) -> VarMetadata {
        VarMetadata {
            source_name: Some(name.to_string()),
            type_name: None,
            byte_range: None,
            is_memory_state: true,
            object_keys: Vec::new(),
        }
    }

    fn add_param(ir: &mut FunctionIR, name: &str) -> VarId {
        let v = ir.new_var(meta(name));
        ir.parameters.push(v);
        v
    }

    fn build_irs() -> Vec<FunctionIR> {
        // fn id(x) { return x; }
        let mut id = FunctionIR::new("id".into());
        let x0 = add_param(&mut id, "x");
        id.set_terminator(
            id.entry_block,
            Terminator::Return {
                src: Some(Operand::Var(x0)),
            },
        );

        // fn clean(x) { return escapeHtml(x); }   ← a REAL sanitizer callee
        let mut clean = FunctionIR::new("clean".into());
        let cx = add_param(&mut clean, "x");
        {
            let b = clean.entry_block;
            let r = clean.new_var(meta("r"));
            let m1 = clean.new_var(mem_meta("m1"));
            clean.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(r),
                    mem_out: m1,
                    mem_in: clean.initial_memory_state,
                    func: "escapeHtml".into(),
                    args: vec![Operand::Var(cx)],
                },
            );
            clean.set_terminator(
                b,
                Terminator::Return {
                    src: Some(Operand::Var(r)),
                },
            );
        }

        // fn main() {
        //   t = getSource();
        //   a = id(t);      db.execute(a);    ← real path: alerts under EVERY config
        //   b = clean(t);   render.html(b);   ← alerts ONLY while escapeHtml is not
        //                                        configured as a sanitizer
        // }
        let mut main = FunctionIR::new("main".into());
        {
            let b = main.entry_block;
            let mem0 = main.initial_memory_state;
            let t = main.new_var(meta("t"));
            let m1 = main.new_var(mem_meta("m1"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(t),
                    mem_out: m1,
                    mem_in: mem0,
                    func: "getSource".into(),
                    args: vec![],
                },
            );
            let a = main.new_var(meta("a"));
            let m2 = main.new_var(mem_meta("m2"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(a),
                    mem_out: m2,
                    mem_in: m1,
                    func: "id".into(),
                    args: vec![Operand::Var(t)],
                },
            );
            let m3 = main.new_var(mem_meta("m3"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m3,
                    mem_in: m2,
                    func: "db.execute".into(),
                    args: vec![Operand::Var(a)],
                },
            );
            let bvar = main.new_var(meta("b"));
            let m4 = main.new_var(mem_meta("m4"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: Some(bvar),
                    mem_out: m4,
                    mem_in: m3,
                    func: "clean".into(),
                    args: vec![Operand::Var(t)],
                },
            );
            let m5 = main.new_var(mem_meta("m5"));
            main.push_instruction(
                b,
                Instruction::CallStatic {
                    dest: None,
                    mem_out: m5,
                    mem_in: m4,
                    func: "render.html".into(),
                    args: vec![Operand::Var(bvar)],
                },
            );
            main.set_terminator(b, Terminator::Return { src: None });
        }
        vec![id, clean, main]
    }

    fn config(sanitizers: &[&str]) -> TaintConfig {
        TaintConfig {
            sources: ["getSource".to_string()].into_iter().collect(),
            sinks: ["db.execute".to_string(), "render.html".to_string()]
                .into_iter()
                .collect(),
            sanitizers: sanitizers.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn leak(irs: Vec<FunctionIR>) -> FxHashMap<String, &'static FunctionIR> {
        irs.into_iter()
            .map(|ir| {
                let leaked: &'static FunctionIR = Box::leak(Box::new(ir));
                (leaked.name.clone(), leaked)
            })
            .collect()
    }

    #[test]
    fn round_trip_preserves_queries() {
        let irs = leak(build_irs());
        let cfg = config(&[]);
        let prog = ProgramSvfg::new(&irs, &cfg);

        let store = SvfgStore::from_program(&prog);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("repo.svfg.json");
        store.save(&path).unwrap();

        let loaded = SvfgStore::load(&path).unwrap();

        // Rebuild under the SAME config with the SAME live IRs.
        let prog2 = loaded.into_program(&irs, &cfg);

        let mut e1 = InterproceduralTaintEngine::new(&prog, &cfg);
        e1.run();
        let mut e2 = InterproceduralTaintEngine::new(&prog2, &cfg);
        e2.run();

        assert_eq!(e1.alerts, e2.alerts, "same config → identical alerts");
        assert_eq!(e1.alerts.len(), 2, "config A: both sink paths alert");

        // Cross edges survived the round trip.
        assert_eq!(prog.cross_edges.len(), prog2.cross_edges.len());
        assert_eq!(prog.topological_order, prog2.topological_order);

        // Summaries were recomputed, not restored.
        let li2 = prog2.function_index("id").unwrap();
        assert!(prog2.functions[li2].summary.is_some());
    }

    #[test]
    fn reload_with_new_sink_config() {
        // The 4.5 headline: build once, cache the STRUCTURE to disk, then
        // re-query that cached store under a DIFFERENT rule set, the stored
        // structure is identical; only summaries/suppression change.
        //
        //   config A (no sanitizers):  db.execute(a) AND render.html(b) alert
        //   config B (escapeHtml):     only db.execute(a) alerts, clean()'s
        //                              summary now suppresses the b-path
        let irs = leak(build_irs());
        let cfg_build = config(&[]);
        let prog_build = ProgramSvfg::new(&irs, &cfg_build);

        // Cache the structure.
        let store = SvfgStore::from_program(&prog_build);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("repo.svfg.json");
        store.save(&path).unwrap();

        // --- Query 1: original rule set (escapeHtml is NOT a sanitizer) ---
        let q1_store = SvfgStore::load(&path).unwrap();
        let prog1 = q1_store.into_program(&irs, &cfg_build);
        let mut e1 = InterproceduralTaintEngine::new(&prog1, &cfg_build);
        e1.run();
        assert_eq!(
            e1.alerts.len(),
            2,
            "config A: both sinks alert; got {:?}",
            e1.alerts
        );

        // --- Query 2: NEW rule set (escapeHtml now sanitizes) ---
        let cfg_new = config(&["escapeHtml"]);
        let q2_store = SvfgStore::load(&path).unwrap();
        let prog2 = q2_store.into_program(&irs, &cfg_new);
        assert_eq!(prog2.functions.len(), prog1.functions.len());

        let mut e2 = InterproceduralTaintEngine::new(&prog2, &cfg_new);
        e2.run();
        assert_eq!(
            e2.alerts.len(),
            1,
            "config B: only the unsanitized db path alerts; got {:?}",
            e2.alerts
        );
        assert!(e2.alerts[0].contains("db.execute"));

        // Reload must equal a from-scratch build under the new config.
        let fresh = ProgramSvfg::new(&irs, &cfg_new);
        let mut ef = InterproceduralTaintEngine::new(&fresh, &cfg_new);
        ef.run();
        assert_eq!(
            e2.alerts, ef.alerts,
            "reload == from-scratch under new config"
        );
    }

    #[test]
    fn determinism_same_program_same_hash() {
        let irs = leak(build_irs());
        let cfg = config(&[]);
        let p1 = ProgramSvfg::new(&irs, &cfg);
        let p2 = ProgramSvfg::new(&irs, &cfg);

        let s1 = SvfgStore::from_program(&p1);
        let s2 = SvfgStore::from_program(&p2);
        assert_eq!(
            s1.content_hash, s2.content_hash,
            "deterministic build → same hash"
        );

        // Byte-identical serialisation (stable corpus cache keys).
        let j1 = serde_json::to_string_pretty(&s1).unwrap();
        let j2 = serde_json::to_string_pretty(&s2).unwrap();
        assert_eq!(j1, j2);
    }

    #[test]
    fn corrupted_payload_is_rejected() {
        let irs = leak(build_irs());
        let cfg = config(&[]);
        let prog = ProgramSvfg::new(&irs, &cfg);
        let store = SvfgStore::from_program(&prog);

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("repo.svfg.json");
        store.save(&path).unwrap();

        // Tamper with the payload WITHOUT breaking its shape (so serde still
        // deserialises it and the hash check is what catches the change):
        // bump the topological order's first entry.
        let data = std::fs::read_to_string(&path).unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&data).unwrap();
        let first = v["parts"]["topological_order"][0].as_u64().unwrap();
        v["parts"]["topological_order"][0] = serde_json::json!(first + 1);
        std::fs::write(&path, serde_json::to_string(&v).unwrap()).unwrap();

        let err = SvfgStore::load(&path).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("hash mismatch"));
    }

    #[test]
    fn wrong_format_version_is_rejected() {
        let irs = leak(build_irs());
        let cfg = config(&[]);
        let prog = ProgramSvfg::new(&irs, &cfg);
        let store = SvfgStore::from_program(&prog);

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("repo.svfg.json");
        store.save(&path).unwrap();

        let data = std::fs::read_to_string(&path).unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&data).unwrap();
        v["format_version"] = serde_json::json!(999);
        std::fs::write(&path, serde_json::to_string(&v).unwrap()).unwrap();

        let err = SvfgStore::load(&path).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        assert!(err.to_string().contains("format mismatch"));
    }

    #[test]
    fn loads_without_live_irs_self_contained() {
        // The store's own IR copies make the graph queryable even when the
        // caller has no live IRs (e.g. a pure cache hit path).
        let irs = leak(build_irs());
        let cfg = config(&[]);
        let prog = ProgramSvfg::new(&irs, &cfg);
        let store = SvfgStore::from_program(&prog);

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("repo.svfg.json");
        store.save(&path).unwrap();

        let loaded = SvfgStore::load(&path).unwrap();
        let empty: FxHashMap<String, &FunctionIR> = FxHashMap::default();
        let prog2 = loaded.into_program(&empty, &cfg);

        let mut e = InterproceduralTaintEngine::new(&prog2, &cfg);
        e.run();
        assert_eq!(
            e.alerts.len(),
            2,
            "self-contained reload still finds both sink paths"
        );
    }

    #[test]
    fn load_is_independent_of_config_at_build_time() {
        // Build under config A, reload the SAME cached structure under config
        // B: summaries/suppression must reflect B, not the config at build
        // time (this is the guarantee that makes per-rule-set queries safe on
        // a shared cache).
        //
        // clean(x) { return escapeHtml(x); } passes taint through under A
        // (escapeHtml is just a call) but sanitizes under B.
        let irs = leak(build_irs());
        let cfg_a = config(&[]);
        let prog_a = ProgramSvfg::new(&irs, &cfg_a);
        let store = SvfgStore::from_program(&prog_a);

        let cfg_b = config(&["escapeHtml"]);
        let prog_b = store.clone().into_program(&irs, &cfg_b);

        // Under config B: clean()'s summary is a sanitizer, param 0 does
        // NOT taint the return.
        let cb = prog_b.function_index("clean").unwrap();
        let sum_b = prog_b.functions[cb].summary.as_ref().unwrap();
        assert!(
            !sum_b.taints_return(0),
            "under config B, clean() sanitizes: param 0 must not taint return"
        );

        // Under config A: the same stored structure yields the OPPOSITE
        // summary (escapeHtml is an unknown call → pass-through).
        let prog_a2 = store.into_program(&irs, &cfg_a);
        let ca = prog_a2.function_index("clean").unwrap();
        let sum_a = prog_a2.functions[ca].summary.as_ref().unwrap();
        assert!(
            sum_a.taints_return(0),
            "under config A, clean() passes taint through: param 0 taints return"
        );

        // And id(x) { return x; } passes taint through under BOTH configs.
        let ib = prog_b.function_index("id").unwrap();
        assert!(
            prog_b.functions[ib]
                .summary
                .as_ref()
                .unwrap()
                .taints_return(0)
        );
    }
}
