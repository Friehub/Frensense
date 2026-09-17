use crate::corpus::semantic::SemanticFilter;
use crate::fingerprint::FunctionFingerprint;

#[derive(Debug, Clone)]
pub struct CorpusPattern {
    pub id: String,
    pub positives: Vec<FunctionFingerprint>,
    pub negatives: Vec<FunctionFingerprint>,
    pub semantic_filter: Option<SemanticFilter>,
    pub observation: Option<String>,
    pub impact: Option<String>,
    pub improvement: Option<String>,
    pub expected_context: Option<crate::context::FileContext>,
    pub cwe: Option<String>,
    pub cvss: Option<f32>,
    pub owasp: Option<String>,
    pub severity: Option<String>,
    pub runtime_probe: Option<String>,
    pub feature_variance: Option<f64>,
    pub min_evidence_dims: Option<usize>,
}

impl From<crate::corpus::bundle::BundlePattern> for CorpusPattern {
    fn from(b: crate::corpus::bundle::BundlePattern) -> Self {
        Self {
            id: b.id,
            positives: b.positives,
            negatives: b.negatives,
            semantic_filter: b.semantic_filter,
            observation: b.observation,
            impact: b.impact,
            improvement: b.improvement,
            expected_context: b.expected_context,
            cwe: b.cwe,
            cvss: b.cvss,
            owasp: b.owasp,
            severity: b.severity,
            runtime_probe: b.runtime_probe,
            feature_variance: b.feature_variance,
            min_evidence_dims: b.min_evidence_dims,
        }
    }
}

impl From<CorpusPattern> for crate::corpus::bundle::BundlePattern {
    fn from(c: CorpusPattern) -> Self {
        Self {
            id: c.id,
            positives: c.positives,
            negatives: c.negatives,
            semantic_filter: c.semantic_filter,
            observation: c.observation,
            impact: c.impact,
            improvement: c.improvement,
            expected_context: c.expected_context,
            cwe: c.cwe,
            cvss: c.cvss,
            owasp: c.owasp,
            severity: c.severity,
            runtime_probe: c.runtime_probe,
            feature_variance: c.feature_variance,
            min_evidence_dims: c.min_evidence_dims,
        }
    }
}
