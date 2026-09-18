pub type FeatureVec = [f64; 14];

// [ngram, ast, sig, ptype, tuse, sem, cf, API, taint, motif, flow, cfOrd, argT, litC]
pub const DEFAULT_WEIGHTS: FeatureVec = [
    0.09, 0.13, 0.06, 0.02, 0.02, 0.09, 0.08, 0.12, 0.10, 0.06, 0.07, 0.03, 0.05, 0.05,
];
