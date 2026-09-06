use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let canary = format!(
        "FRENSENSE_{}",
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    );
    ProbeTemplate {
        category: "xss",
        baseline_payload: "test",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("<script>document.write('{canary}')</script>"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "Reflected XSS via script tag — canary in response confirms injection"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("\"'><img src=x onerror=alert({canary})>"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "XSS via img onerror handler — bypasses simple filters".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("javascript:alert('{canary}')"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "XSS via javascript: URL scheme in href/src attributes".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("<svg onload=alert({canary})>"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "XSS via SVG onload handler".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "<script>alert(1)</script>".to_string(),
                oracle: OracleKind::ErrorPattern {
                    patterns: vec!["<script>".to_string(), "alert(1)".to_string()],
                },
                risk: ProbeRisk::Safe,
                description: "Reflected XSS — script tag reflected in response".to_string(),
            },
        ],
    }
}
