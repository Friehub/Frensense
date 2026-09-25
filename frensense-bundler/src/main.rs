// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Frensense bundler CLI: builds a `.frc` facts bundle from positive/negative
//! corpus pairs.
//!
//! Usage: frensense-bundler <corpus_dir> <output.frc> [--facts]

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").map_or_else(|_| env::current_dir().unwrap(), PathBuf::from);
    let workspace_root = manifest_dir.parent().unwrap();

    let args: Vec<String> = env::args().collect();
    let corpus_dir = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("corpus").join("targets"));
    let output_path = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("frensense-corpus.frc"));

    if let Err(e) = run_facts_pipeline(&corpus_dir, &output_path) {
        eprintln!("Error in facts pipeline: {e}");
        std::process::exit(1);
    }
}

/// §9 facts pipeline entry: uses the engine's shared harness + fact table.
fn run_facts_pipeline(corpus_dir: &Path, output_path: &Path) -> Result<(), String> {
    use frensense_engine::analysis::taint::config::TaintConfig;
    use frensense_engine::analysis::taint::facts::FactTable;
    use frensense_lang::spec_for_ext;

    // Built-in config + fact table from every registered language spec:
    // the corpus may contain any supported language.
    let mut config = TaintConfig::default();
    let mut builtin = FactTable::default();
    for spec in frensense_lang::all_specs() {
        let c = frensense_engine::analysis::taint::facts::config_from_spec(spec);
        config.sources.extend(c.sources);
        config.sinks.extend(c.sinks);
        config.sanitizers.extend(c.sanitizers);
        builtin.merge(&frensense_engine::analysis::taint::facts::fact_table_from_spec(spec));
    }
    let _ = spec_for_ext;

    // Corpus-owned seed facts (deployment-specific knowledge) merge over
    // the spec tables, the fact extractor must see the same base table
    // the scanner sees.
    for candidate in [
        std::env::var("FRENSENSE_SEED_FACTS")
            .ok()
            .map(PathBuf::from),
        Some(PathBuf::from("corpus/facts/seed_facts.json")),
        Some(corpus_dir.join("../facts/seed_facts.json")),
    ]
    .into_iter()
    .flatten()
    {
        if let Err(e) = frensense_engine::analysis::taint::facts::seed::SeedFacts::load_and_apply(
            &candidate,
            &mut builtin,
        ) {
            eprintln!("[warn] {e}");
            break;
        }
    }

    let (bytes, facts) =
        frensense_bundler::builder::build_facts_bundle(corpus_dir, &config, &builtin)?;

    // Round-trip verify before writing.
    let loaded = frensense_bundler::format::load_bundle(&bytes)?;
    eprintln!(
        "[facts] round-trip OK: {} learned facts in bundle",
        loaded.learned_facts.len()
    );

    for f in &facts {
        let call = match &f.entry {
            frensense_engine::analysis::taint::facts::LearnedFactEntry::Sink { call, .. } => {
                format!("sink:{call}")
            }
            frensense_engine::analysis::taint::facts::LearnedFactEntry::Sanitizer {
                call,
                kind,
                ..
            } => {
                format!("sanitizer:{call}({kind})")
            }
            frensense_engine::analysis::taint::facts::LearnedFactEntry::Check {
                rule,
                call,
                ..
            } => {
                format!("check:{rule}({call})")
            }
        };
        eprintln!("  [{}] {} ← {}", f.status, call, f.families.join(", "));
    }

    std::fs::write(output_path, &bytes)
        .map_err(|e| format!("write {}: {e}", output_path.display()))?;
    eprintln!(
        "Facts bundle written to {} ({} bytes)",
        output_path.display(),
        bytes.len()
    );
    Ok(())
}
