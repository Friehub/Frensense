// SPDX-License-Identifier: MIT

//! Near-duplicate function clustering using Union-Find.
//!
//! Groups functions into clusters based on MinHash similarity.
//! Identifies inconsistent implementations within clusters.

use crate::engine::fingerprint::FunctionFingerprint;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionCluster {
    pub id: usize,
    pub members: Vec<ClusterMember>,
    pub has_inconsistency: bool,
}

#[derive(Debug, Clone)]
pub struct ClusterMember {
    pub fingerprint: FunctionFingerprint,
    pub cluster_role: ClusterRole,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClusterRole {
    /// This function is identical to others in the cluster
    Consistent,
    /// This function differs from others (potential bug)
    Inconsistent,
    /// This is the "safe" version (has sanitizer/validation)
    Safe,
    /// This is the "unsafe" version (missing validation)
    Unsafe,
}

/// Union-Find data structure for clustering.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else {
            self.parent[y_root] = x_root;
            self.rank[x_root] += 1;
        }
    }
}

/// Cluster functions by near-duplicate similarity.
pub fn cluster_functions(
    fingerprints: &[FunctionFingerprint],
    similarity_threshold: f64,
) -> Vec<FunctionCluster> {
    if fingerprints.is_empty() {
        return Vec::new();
    }

    let n = fingerprints.len();
    let mut uf = UnionFind::new(n);

    // Union functions that are similar enough
    for i in 0..n {
        for j in (i + 1)..n {
            let sim = frensense_engine::minhash::approximate_jaccard(
                &fingerprints[i].ngram_hashes,
                &fingerprints[j].ngram_hashes,
            );
            if sim >= similarity_threshold {
                uf.union(i, j);
            }
        }
    }

    // Group by root
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        groups.entry(root).or_default().push(i);
    }

    // Build clusters
    let mut clusters = Vec::new();
    let mut cluster_id = 0;

    for (_root, members) in groups {
        if members.len() < 2 {
            continue; // Skip singletons
        }

        let cluster_members: Vec<ClusterMember> = members
            .iter()
            .map(|&idx| {
                let fp = &fingerprints[idx];
                let role = classify_member_role(fp, &fingerprints, &members);
                ClusterMember {
                    fingerprint: fp.clone(),
                    cluster_role: role,
                }
            })
            .collect();

        let has_inconsistency = cluster_members
            .iter()
            .any(|m| m.cluster_role == ClusterRole::Inconsistent);

        clusters.push(FunctionCluster {
            id: cluster_id,
            members: cluster_members,
            has_inconsistency,
        });

        cluster_id += 1;
    }

    clusters
}

/// Classify a function's role within its cluster.
fn classify_member_role(
    fp: &FunctionFingerprint,
    all_fps: &[FunctionFingerprint],
    cluster_indices: &[usize],
) -> ClusterRole {
    // Check if this function has validation/sanitization patterns
    let has_validation = fp.function_name.contains("valid")
        || fp.function_name.contains("sanitiz")
        || fp.function_name.contains("check")
        || fp.function_name.contains("verify");

    // Check if this function has dangerous patterns
    let has_danger = fp.function_name.contains("exec")
        || fp.function_name.contains("eval")
        || fp.function_name.contains("raw")
        || fp.function_name.contains("unsafe");

    if has_validation && !has_danger {
        ClusterRole::Safe
    } else if has_danger && !has_validation {
        ClusterRole::Unsafe
    } else {
        // Check if this member differs significantly from others in the cluster
        let avg_sim: f64 = cluster_indices
            .iter()
            .filter(|&&other_idx| {
                let other = &all_fps[other_idx];
                other.function_name != fp.function_name
            })
            .map(|&other_idx| {
                frensense_engine::minhash::approximate_jaccard(
                    &fp.ngram_hashes,
                    &all_fps[other_idx].ngram_hashes,
                )
            })
            .sum::<f64>()
            / (cluster_indices.len().saturating_sub(1)) as f64;

        if avg_sim < 0.85 {
            ClusterRole::Inconsistent
        } else {
            ClusterRole::Consistent
        }
    }
}

/// Generate advisories from clusters with inconsistencies.
pub fn cluster_to_advisories(
    clusters: &[FunctionCluster],
) -> Vec<crate::Advisory> {
    let mut advisories = Vec::new();

    for cluster in clusters {
        if !cluster.has_inconsistency {
            continue;
        }

        let inconsistent: Vec<&ClusterMember> = cluster
            .members
            .iter()
            .filter(|m| m.cluster_role == ClusterRole::Inconsistent)
            .collect();

        let safe: Vec<&ClusterMember> = cluster
            .members
            .iter()
            .filter(|m| m.cluster_role == ClusterRole::Safe)
            .collect();

        for member in &inconsistent {
            let safe_names: Vec<&str> = safe
                .iter()
                .map(|m| m.fingerprint.function_name.as_str())
                .collect();

            let advisory = crate::Advisory::bare(
                "NEAR_DUPLICATE_INCONSISTENT",
                crate::Severity::Warning,
                crate::FileId(0),
                std::path::Path::new(&member.fingerprint.file_path),
                format!(
                    "Function '{}' is structurally similar to other functions in cluster {} but differs in implementation.{}",
                    member.fingerprint.function_name,
                    cluster.id,
                    if !safe_names.is_empty() {
                        format!(" Safe versions exist: {}", safe_names.join(", "))
                    } else {
                        String::new()
                    }
                ),
            )
            .with_confidence(0.7)
            .with_line(u32::try_from(member.fingerprint.line).unwrap_or(u32::MAX))
            .with_content(member.fingerprint.function_name.clone())
            .with_impact("Inconsistent implementations of the same logic may indicate a missing security fix in one copy.")
            .with_improvement("Ensure all copies apply the same validation/sanitization. Consider extracting shared logic.")
            .with_tags(["consistency", "duplicate", "cluster"]);

            advisories.push(advisory);
        }
    }

    advisories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(2), uf.find(3));
        assert_ne!(uf.find(0), uf.find(2));
    }
}
