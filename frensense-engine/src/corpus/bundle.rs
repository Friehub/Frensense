// SPDX-License-Identifier: MIT

//! # FRC1 — Frensense Reference Corpus Bundle Format
//!
//! A pre-compiled binary format that embeds corpus fingerprints directly into the
//! engine binary, eliminating runtime file I/O and tree-sitter parsing at startup.
//!
//! ## Why bundles exist
//!
//! The corpus is a set of positive/negative example pairs used for pattern matching.
//! At 400 patterns (4 functions × 2 polarities = 3,200 fingerprints), parsing source
//! files at startup becomes slow. The bundle pre-computes all fingerprints and serializes
//! them into a compact binary blob (~300KB–1MB compressed).
//!
//! ## Binary layout
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │ u32 LE  header_length                       │  4 bytes
//! ├─────────────────────────────────────────────┤
//! │ BundleHeader (bincode-serialized)           │  ~52 bytes
//! │   magic:        [u8; 4]  = b"FRC1"          │
//! │   version:      u32      = 1                │
//! │   pattern_count: u32                         │
//! │   checksum:     [u8; 32] = blake3(data)     │
//! ├─────────────────────────────────────────────┤
//! │ Vec<BundlePattern> (bincode-serialized)      │  variable
//! │   Each pattern:                              │
//! │     id: String                               │
//! │     positives: Vec<FunctionFingerprint>      │
//! │     negatives: Vec<FunctionFingerprint>      │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! - **magic**: Must be `b"FRC1"`. Engine rejects unknown magic.
//! - **version**: Must be ≤ engine's `BUNDLE_VERSION`. Engine rejects newer bundles
//!   to avoid deserialization mismatches when the `FunctionFingerprint` struct changes.
//! - **checksum**: blake3 hash of the serialized `Vec<BundlePattern>` data. Verified on load.
//! - **`FunctionFingerprint`**: Contains n-gram hashes, structural markers, signature n-grams,
//!   param type n-grams, type usages, weighted n-gram hashes (IDF), language, function name.
//!   No source text is stored — fingerprints are one-way hashes.
//!
//! ## Building the bundle
//!
//! ```sh
//! cargo run --bin build-corpus-bundle
//! ```
//!
//! Reads `corpus/targets/*_positive.*` and `*_negative.*`, extracts all functions per file,
//! serializes to bincode, writes `frensense-corpus.frc` to the repo root.
//!
//! ## Loading the bundle
//!
//! The engine loads the bundle via `PatternRegistry::load_from_bundle(bytes)`. If the
//! bundle is missing, corrupted, or has a newer version, it falls back to loading from
//! the source `corpus/targets/` directory.
//!
//! The binary embeds the bundle via `include_bytes!("../frensense-corpus.frc")` and passes
//! it to the engine at startup.
//!
//! ## Adding a new pattern
//!
//! 1. Create `corpus/targets/{language}_{name}_positive.{ext}` with buggy code
//! 2. Create `corpus/targets/{language}_{name}_negative.{ext}` with fixed code
//! 3. Run `cargo run --bin build-corpus-bundle` to rebuild the bundle
//! 4. Commit both the source files and the updated `frensense-corpus.frc`

use std::path::Path;

use crate::corpus::loader::load_corpus;
use crate::fingerprint::FunctionFingerprint;

pub const BUNDLE_MAGIC: &[u8; 4] = b"FRC1";
pub const BUNDLE_VERSION: u32 = 4;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BundleHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub pattern_count: u32,
    pub checksum: [u8; 32],
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BundlePattern {
    pub id: String,
    pub positives: Vec<FunctionFingerprint>,
    pub negatives: Vec<FunctionFingerprint>,
    #[serde(default)]
    pub semantic_filter: Option<crate::corpus::semantic::SemanticFilter>,
    #[serde(default)]
    pub observation: Option<String>,
    #[serde(default)]
    pub impact: Option<String>,
    #[serde(default)]
    pub improvement: Option<String>,
    #[serde(default)]
    pub expected_context: Option<crate::context::FileContext>,
    #[serde(default)]
    pub cwe: Option<String>,
    #[serde(default)]
    pub cvss: Option<f32>,
    #[serde(default)]
    pub owasp: Option<String>,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub runtime_probe: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Bundle {
    pub header: BundleHeader,
    pub patterns: Vec<BundlePattern>,
}

/// Internal serialization wrapper for the data section of an FRC bundle.
/// Stores both patterns and pre-computed API-call IDF weights.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct BundlePayload {
    patterns: Vec<BundlePattern>,
    /// Pre-computed API-call IDF weights as a sorted `(hash, idf_score)` vec.
    /// Sorted for deterministic serialization and byte-for-byte reproducible bundles.
    #[serde(default)]
    api_idf_weights: Vec<(u64, f32)>,
    /// Per-category learned feature weights (8-d vector per category).
    /// Trained at build time via logistic regression on corpus positive/negative pairs.
    #[serde(default)]
    category_weights: Vec<(String, [f64; 11])>,
    /// Auto-derived semantic filter suggestions (import + call exclusivity).
    #[serde(default)]
    auto_filter_stats: Vec<(String, Vec<String>, Vec<String>, Vec<String>, String, Vec<String>, Vec<String>)>,
    /// Per-pattern sigmoid calibration params (A, B).
    #[serde(default)]
    pattern_calibration: Vec<(String, f32, f32)>,
}

/// Returned by `load_bundle`; carries both the deserialized patterns and
/// pre-computed weights so callers can skip recomputation.
pub struct LoadedBundle {
    pub patterns: Vec<BundlePattern>,
    pub api_idf_weights: Vec<(u64, f32)>,
    pub category_weights: Vec<(String, [f64; 11])>,
    pub auto_filter_stats: Vec<(String, Vec<String>, Vec<String>, Vec<String>, String, Vec<String>, Vec<String>)>,
    pub pattern_calibration: Vec<(String, f32, f32)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ManifestEntry {
    path: String,
    mtime: u64,
    content_hash: [u8; 32],
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct Manifest {
    entries: Vec<ManifestEntry>,
}

impl Manifest {
    fn load(path: &Path) -> Self {
        let Ok(content) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }

    fn save(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, content).map_err(|e| e.to_string())
    }

    fn update_entry(&mut self, path: String, mtime: u64, content_hash: [u8; 32]) {
        self.entries.retain(|e| e.path != path);
        self.entries.push(ManifestEntry {
            path,
            mtime,
            content_hash,
        });
    }
}

/// Build a bundle from a corpus directory.
pub fn build_bundle(corpus_dir: &Path) -> Result<Vec<u8>, String> {
    let patterns = load_corpus(corpus_dir)?;

    let bundle_patterns: Vec<BundlePattern> = patterns
        .into_iter()
        .map(|p| BundlePattern {
            id: p.id,
            positives: p.positives,
            negatives: p.negatives,
            semantic_filter: p.semantic_filter,
            observation: p.observation,
            impact: p.impact,
            improvement: p.improvement,
            expected_context: p.expected_context,
            cwe: p.cwe,
            cvss: p.cvss,
            owasp: p.owasp,
            severity: p.severity,
            runtime_probe: p.runtime_probe,
        })
        .collect();

    build_bundle_from_patterns(&bundle_patterns)
}

/// Compute API-call IDF weights from a slice of bundle patterns.
/// Mirrors the logic in `PatternRegistry::compute_api_idf` but operates on
/// `BundlePattern` so it can run at build time without a live registry.
fn compute_bundle_api_idf(patterns: &[BundlePattern]) -> Vec<(u64, f32)> {
    let total = patterns.len() as f32;
    if total == 0.0 {
        return Vec::new();
    }
    let mut api_doc_freq: rustc_hash::FxHashMap<u64, f32> = rustc_hash::FxHashMap::default();
    for pattern in patterns {
        let mut seen: rustc_hash::FxHashSet<u64> = rustc_hash::FxHashSet::default();
        for fp in &pattern.positives {
            for &call in &fp.api_calls {
                if seen.insert(call) {
                    *api_doc_freq.entry(call).or_insert(0.0) += 1.0;
                }
            }
        }
    }
    let mut weights: Vec<(u64, f32)> = api_doc_freq
        .into_iter()
        .map(|(hash, df)| (hash, (total / df).ln()))
        .collect();
    weights.sort_unstable_by_key(|&(hash, _)| hash);
    weights
}

/// Build a bundle from pre-built `BundlePatterns`.
pub fn build_bundle_from_patterns(patterns: &[BundlePattern]) -> Result<Vec<u8>, String> {
    // Pre-compute API IDF at build time so loaders can skip recomputation (~100 ms saving)
    let api_idf_weights = compute_bundle_api_idf(patterns);

    // Learn per-category feature weights from positive/negative pairs
    let corpus_patterns: Vec<crate::corpus::loader::CorpusPattern> = patterns
        .iter()
        .map(|bp| crate::corpus::loader::CorpusPattern {
            id: bp.id.clone(),
            positives: bp.positives.clone(),
            negatives: bp.negatives.clone(),
            semantic_filter: bp.semantic_filter.clone(),
            observation: bp.observation.clone(),
            impact: bp.impact.clone(),
            improvement: bp.improvement.clone(),
            expected_context: bp.expected_context.clone(),
            cwe: bp.cwe.clone(),
            cvss: bp.cvss,
            owasp: bp.owasp.clone(),
            severity: bp.severity.clone(),
            runtime_probe: bp.runtime_probe.clone(),
        })
        .collect();
    let category_weights_vec: Vec<(String, [f64; 11])> =
        crate::pattern::weight_learner::learn_category_weights(&corpus_patterns)
            .into_iter()
            .collect();

    // Compute auto-derived semantic filter suggestions
    // We need source text for each pattern to extract imports and call targets
    let corpus_dir = std::env::current_dir()
        .map(|d| d.join("corpus").join("targets"))
        .unwrap_or_default();
    // Recursively find a corpus file by its pattern ID and variant.
    fn find_corpus_file(id: &str, variant: &str, dir: &Path) -> Option<String> {
        for ext in &["ts", "tsx", "js", "jsx", "rs"] {
            let target = format!("{}_{}.{}", id, variant, ext);
            let result = find_file_recursive(dir, &target);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn find_file_recursive(dir: &Path, target: &str) -> Option<String> {
        let Ok(entries) = std::fs::read_dir(dir) else { return None; };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(src) = find_file_recursive(&path, target) {
                    return Some(src);
                }
            } else if path.is_file() {
                if path.file_name().and_then(|n| n.to_str()) == Some(target) {
                    return std::fs::read_to_string(&path).ok();
                }
            }
        }
        None
    }

    let mut pattern_source_texts = std::collections::HashMap::new();
    for bp in patterns {
        // Read positive and up to 4 negative source files for auto-filter learning.
        // Files may be in any subdirectory under corpus_dir.
        if let Some(src) = find_corpus_file(&bp.id, "positive", corpus_dir.as_path()) {
            pattern_source_texts.insert(bp.id.clone(), src);
        }
        for variant in &["negative", "negative2", "negative3", "negative4"] {
            if let Some(src) = find_corpus_file(&bp.id, variant, corpus_dir.as_path()) {
                pattern_source_texts.insert(format!("{}_neg", bp.id), src);
            }
        }
    }
    let auto_stats = crate::auto_filter::compute_auto_filters(patterns, &pattern_source_texts);
    // Serialize auto-derived filter stats.  Each entry is:
    // (pid, imports, calls, excludes_call, function_name_regex, excludes_nodes, excludes_fnames)
    let auto_filter_stats: Vec<(String, Vec<String>, Vec<String>, Vec<String>, String, Vec<String>, Vec<String>)> = {
        let mut v = Vec::new();
        let all_pids: std::collections::HashSet<&str> = patterns.iter().map(|p| p.id.as_str()).collect();
        for pid in &all_pids {
            let imports = auto_stats.contains_import.get(*pid).cloned().unwrap_or_default();
            let calls = auto_stats.contains_call_to.get(*pid).cloned().unwrap_or_default();
            let excl_calls = auto_stats.excludes_call.get(*pid).cloned().unwrap_or_default();
            let fn_re = auto_stats.function_name_regex.get(*pid).cloned().unwrap_or_default();
            let excl_nodes = auto_stats.excludes_node_type.get(*pid).cloned().unwrap_or_default();
            let excl_fnames = auto_stats.excludes_function_name.get(*pid).cloned().unwrap_or_default();
            if !imports.is_empty() || !calls.is_empty() || !excl_calls.is_empty() || !fn_re.is_empty() {
                v.push((pid.to_string(), imports, calls, excl_calls, fn_re, excl_nodes, excl_fnames));
            }
        }
        v
    };

    // Train per-pattern calibration sigmoids
    let pattern_cal: Vec<(String, f32, f32)> =
        crate::per_pattern_calibration::train_per_pattern_calibration(&corpus_patterns)
            .into_iter()
            .map(|(k, (a, b))| (k, a, b))
            .collect();

    let payload = BundlePayload {
        patterns: patterns.to_vec(),
        api_idf_weights,
        category_weights: category_weights_vec,
        auto_filter_stats,
        pattern_calibration: pattern_cal,
    };
    let data = bincode::serialize(&payload).map_err(|e| e.to_string())?;

    let checksum = blake3::hash(&data);
    let header = BundleHeader {
        magic: *BUNDLE_MAGIC,
        version: BUNDLE_VERSION,
        pattern_count: patterns.len() as u32,
        checksum: *checksum.as_bytes(),
    };

    let mut output = Vec::new();
    let header_bytes = bincode::serialize(&header).map_err(|e| e.to_string())?;
    output.extend_from_slice(&(header_bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(&header_bytes);
    output.extend_from_slice(&data);

    Ok(output)
}

/// Build a bundle incrementally, only reprocessing changed files.
pub fn build_bundle_incremental(corpus_dir: &Path) -> Result<Vec<u8>, String> {
    let manifest_path = corpus_dir.join(".bundle_manifest.toml");
    let mut manifest = Manifest::load(&manifest_path);

    let patterns = load_corpus(corpus_dir)?;

    let bundle_patterns: Vec<BundlePattern> = patterns
        .into_iter()
        .map(|p| BundlePattern {
            id: p.id,
            positives: p.positives,
            negatives: p.negatives,
            semantic_filter: p.semantic_filter,
            observation: p.observation,
            impact: p.impact,
            improvement: p.improvement,
            expected_context: p.expected_context,
            cwe: p.cwe,
            cvss: p.cvss,
            owasp: p.owasp,
            severity: p.severity,
            runtime_probe: p.runtime_probe,
        })
        .collect();

    let data = bincode::serialize(&bundle_patterns).map_err(|e| e.to_string())?;

    let checksum = blake3::hash(&data);
    let header = BundleHeader {
        magic: *BUNDLE_MAGIC,
        version: BUNDLE_VERSION,
        pattern_count: bundle_patterns.len() as u32,
        checksum: *checksum.as_bytes(),
    };

    let mut output = Vec::new();
    let header_bytes = bincode::serialize(&header).map_err(|e| e.to_string())?;
    output.extend_from_slice(&(header_bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(&header_bytes);
    output.extend_from_slice(&data);

    // Update manifest with current file hashes
    if let Ok(entries) = std::fs::read_dir(corpus_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if std::path::Path::new(name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("toml"))
                    && name != ".bundle_manifest.toml"
                {
                    continue;
                }
                if let Ok(metadata) = std::fs::metadata(&path) {
                    let mtime = metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map_or(0, |d| d.as_secs());
                    if let Ok(content) = std::fs::read(&path) {
                        let content_hash = blake3::hash(&content).into();
                        manifest.update_entry(
                            path.to_string_lossy().to_string(),
                            mtime,
                            content_hash,
                        );
                    }
                }
            }
        }
    }

    manifest.save(&manifest_path)?;

    Ok(output)
}

/// Load patterns and API IDF weights from a bundle byte slice.
pub fn load_bundle(bytes: &[u8]) -> Result<LoadedBundle, String> {
    if bytes.len() < 4 {
        return Err("bundle too small".to_string());
    }

    let header_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    if bytes.len() < 4 + header_len {
        return Err("bundle truncated (header)".to_string());
    }

    let header: BundleHeader =
        bincode::deserialize(&bytes[4..4 + header_len]).map_err(|e| e.to_string())?;

    if header.magic != *BUNDLE_MAGIC {
        return Err(format!(
            "invalid magic: expected {:?}, got {:?}",
            BUNDLE_MAGIC, header.magic
        ));
    }
    if header.version > BUNDLE_VERSION {
        return Err(format!(
            "bundle version {} > engine version {}",
            header.version, BUNDLE_VERSION
        ));
    }

    let data_start = 4 + header_len;
    if bytes.len() < data_start {
        return Err("bundle truncated (data)".to_string());
    }

    let data = &bytes[data_start..];
    let checksum = blake3::hash(data);
    if *checksum.as_bytes() != header.checksum {
        return Err("checksum mismatch".to_string());
    }

    // Deserialize as BundlePayload (version ≥ 3) with graceful fallback for v2 bundles
    // that serialized a bare Vec<BundlePattern>. bincode will fail on the wrong shape,
    // so we try the new format first and fall back to the legacy flat vec on error.
    let (patterns, api_idf_weights, category_weights, auto_filter_stats, pattern_calibration): (
        Vec<BundlePattern>,
        Vec<(u64, f32)>,
        Vec<(String, [f64; 11])>,
        Vec<(String, Vec<String>, Vec<String>, Vec<String>, String, Vec<String>, Vec<String>)>,
        Vec<(String, f32, f32)>,
    ) = if let Ok(payload) = bincode::deserialize::<BundlePayload>(data) {
        (payload.patterns, payload.api_idf_weights, payload.category_weights, payload.auto_filter_stats, payload.pattern_calibration)
    } else {
        // Legacy v2 bundle — no embedded metrics; caller will recompute
        let patterns: Vec<BundlePattern> =
            bincode::deserialize(data).map_err(|e| e.to_string())?;
        (patterns, Vec::new(), Vec::new(), Vec::new(), Vec::new())
    };

    if patterns.len() != header.pattern_count as usize {
        return Err(format!(
            "pattern count mismatch: header says {} but found {}",
            header.pattern_count,
            patterns.len()
        ));
    }

    Ok(LoadedBundle {
        patterns,
        api_idf_weights,
        category_weights,
        auto_filter_stats,
        pattern_calibration,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("test_positive.rs"), "fn foo() -> i32 { 1 }").unwrap();
        std::fs::write(dir.path().join("test_negative.rs"), "fn foo() -> i32 { 2 }").unwrap();

        let bytes = build_bundle(dir.path()).unwrap();
        let loaded = load_bundle(&bytes).unwrap();
        assert_eq!(loaded.patterns.len(), 1);
        assert_eq!(loaded.patterns[0].id, "test");
        assert!(!loaded.patterns[0].positives.is_empty());
        assert!(!loaded.patterns[0].negatives.is_empty());
    }

    #[test]
    fn test_bundle_version_check() {
        let bytes = vec![b'F', b'R', b'C', b'1']; // Too small for header
        assert!(load_bundle(&bytes).is_err());
    }

    #[test]
    fn test_bundle_invalid_magic() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("x_positive.rs"), "fn x() {}").unwrap();
        std::fs::write(dir.path().join("x_negative.rs"), "fn x() { let y = 1; }").unwrap();

        let mut bytes = build_bundle(dir.path()).unwrap();
        // Corrupt magic
        bytes[6] = b'X';
        assert!(load_bundle(&bytes).is_err());
    }
}
