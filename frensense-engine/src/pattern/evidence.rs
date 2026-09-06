// SPDX-License-Identifier: MIT

//! Match evidence: per-dimension breakdown of why a corpus match scored as it did.

/// Per-dimension evidence collected during a pattern match.
/// All fields are in [0, 1] range (Jaccard similarity for set fields).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MatchEvidence {
    /// N-gram (text token) similarity to the closest positive example.
    pub ngram_sim: f64,
    /// AST skeleton similarity (LCS-based tree edit distance complement).
    pub ast_sim: f64,
    /// Function signature n-gram similarity (parameter names and types).
    pub signature_sim: f64,
    /// Control-flow path similarity.
    pub control_flow_sim: f64,
    /// API call similarity (literal call names).
    pub api_sim: f64,
    /// Motif similarity (abstract sink/source family).
    pub motif_sim: f64,
    /// Data-flow path similarity (UserInputSource -> sink chains).
    pub flow_sim: Option<f64>,
    /// Semantic marker category overlap.
    pub semantic_sim: f64,
    /// Similarity to the closest negative (higher = more like safe code).
    pub negative_sim: f64,
    /// Which API calls from the pattern were found in the candidate.
    pub matched_calls: Vec<String>,
    /// Which API calls from the pattern were NOT found in the candidate.
    pub missing_calls: Vec<String>,
    /// Which motifs from the pattern were matched.
    pub matched_motifs: Vec<String>,
    /// Whether a data-flow taint path from source to sink was found.
    pub has_taint_path: bool,
    /// The specific positive example index that yielded the best score.
    pub best_positive_index: usize,
}
