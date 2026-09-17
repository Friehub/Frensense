// SPDX-License-Identifier: MIT

//! Sink/source motif definitions.
//!
//! A motif is a named group of semantically equivalent API calls.
//! At fingerprint time, calls are hashed under the motif name instead of
//! (or in addition to) their literal name. This makes patterns trained on
//! `exec()` automatically match `spawn()`, `Command::new()` etc.

use rustc_hash::FxHashMap;
use std::sync::LazyLock;

/// A motif: a canonical name plus the set of concrete calls it covers.
#[derive(Debug, Clone)]
pub struct Motif {
    pub name: &'static str,
    pub members: &'static [&'static str],
}

/// All registered motifs.

/// Build a lookup table from call name → motif canonical name.
fn build_motif_lookup() -> FxHashMap<String, &'static str> {
    let mut map = FxHashMap::default();
    for spec in frensense_lang::registry::all_specs() {
        for &(member, motif_name) in spec.known_motif_members() {
            map.insert(member.to_string(), motif_name);
            if let Some(pos) = member.rfind("::").or_else(|| member.rfind('.')) {
                let seg = &member[pos + 1..];
                map.entry(seg.to_string()).or_insert(motif_name);
            }
        }
    }
    map
}

/// Cached motif lookup table, built once at first access.
pub static MOTIF_LOOKUP: LazyLock<FxHashMap<String, &'static str>> =
    LazyLock::new(build_motif_lookup);
