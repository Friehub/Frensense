// SPDX-License-Identifier: MIT
//! Builds the corpus fingerprint bundle (frensense-corpus.frc).
//!
//! Usage: cargo run --bin build-corpus-bundle
//!
//! Reads corpus/targets/, extracts fingerprints, writes frensense-corpus.frc.

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let corpus_dir = manifest_dir.join("corpus").join("targets");
    let output_path = manifest_dir.join("frensense-corpus.frc");

    eprintln!("Building corpus bundle from {}...", corpus_dir.display());

    let bytes = match frensense_engine::corpus::bundle::build_bundle(&corpus_dir) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error building bundle: {e}");
            std::process::exit(1);
        }
    };

    let patterns = match frensense_engine::corpus::bundle::load_bundle(&bytes) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error verifying bundle: {e}");
            std::process::exit(1);
        }
    };

    let total_fingerprints: usize = patterns
        .iter()
        .map(|p| p.positives.len() + p.negatives.len())
        .sum();

    std::fs::write(&output_path, &bytes).unwrap_or_else(|e| {
        eprintln!("Error writing bundle: {e}");
        std::process::exit(1);
    });

    eprintln!(
        "Bundle written to {} ({} bytes, {} patterns, {} fingerprints)",
        output_path.display(),
        bytes.len(),
        patterns.len(),
        total_fingerprints,
    );
}
