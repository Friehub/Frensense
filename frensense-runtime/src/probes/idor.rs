use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let _canary = format!(
        "FRENSENSE_{}",
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    );
    ProbeTemplate {
        category: "idor",
        baseline_payload: "1",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "2".to_string(),
                oracle: OracleKind::DifferentialResponse {
                    min_divergence_score: 0.3,
                },
                risk: ProbeRisk::Safe,
                description: "IDOR — increment resource ID, check if unauthorized data is returned"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "1000000".to_string(),
                oracle: OracleKind::StatusCodeChange {
                    expected_original: 200,
                    trigger_different: true,
                },
                risk: ProbeRisk::Safe,
                description: "IDOR — high resource ID may return 403/404 if properly protected"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "-1".to_string(),
                oracle: OracleKind::StatusCodeChange {
                    expected_original: 200,
                    trigger_different: true,
                },
                risk: ProbeRisk::Safe,
                description: "IDOR — negative ID may trigger different error handling".to_string(),
            },
        ],
    }
}
