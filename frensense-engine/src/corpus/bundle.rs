use crate::auto_filter::AutoFilterEntry;
use crate::fingerprint::FunctionFingerprint;
use frensense_frc::BundleHeader;

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
    #[serde(default)]
    pub feature_variance: Option<f64>,
    #[serde(default)]
    pub min_evidence_dims: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Bundle {
    pub header: BundleHeader,
    pub patterns: Vec<BundlePattern>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BundlePayload {
    pub patterns: Vec<BundlePattern>,
    #[serde(default)]
    pub api_idf_weights: Vec<(u64, f32)>,
    #[serde(default)]
    pub auto_filter_stats: Vec<AutoFilterEntry>,
}

pub struct LoadedBundle {
    pub patterns: Vec<BundlePattern>,
    pub api_idf_weights: Vec<(u64, f32)>,
    pub auto_filter_stats: Vec<AutoFilterEntry>,
}

pub fn load_bundle(bytes: &[u8]) -> Result<LoadedBundle, String> {
    let (header, patterns, api_idf_weights, auto_filter_stats) =
        match frensense_frc::read_bundle::<BundlePayload>(bytes) {
            Ok((h, payload)) => (
                h,
                payload.patterns,
                payload.api_idf_weights,
                payload.auto_filter_stats,
            ),
            Err(e) => match frensense_frc::read_bundle::<Vec<BundlePattern>>(bytes) {
                Ok((h, patterns)) => (h, patterns, Vec::new(), Vec::new()),
                Err(_) => return Err(format!("Failed to deserialize bundle: {}", e)),
            },
        };

    if patterns.len() < header.pattern_count as usize {
        tracing::warn!(
            "bundle pattern count mismatch (expected {}, loaded {})",
            header.pattern_count,
            patterns.len()
        );
    }

    Ok(LoadedBundle {
        patterns,
        api_idf_weights,
        auto_filter_stats,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_version_check() {
        let bytes = vec![b'F', b'R', b'C', b'1'];
        assert!(load_bundle(&bytes).is_err());
    }
}
