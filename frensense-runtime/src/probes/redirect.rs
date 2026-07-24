use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template(canary_host: &str) -> ProbeTemplate {
    ProbeTemplate {
        category: "redirect",
        baseline_payload: "/dashboard",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("https://{canary_host}/probe"),
                oracle: OracleKind::RedirectToCanary {
                    canary_host: canary_host.to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Open redirect to external host — Location header confirms".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("//{canary_host}/probe"),
                oracle: OracleKind::RedirectToCanary {
                    canary_host: canary_host.to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Protocol-relative redirect — bypasses http:// prefix check".to_string(),
            },
        ],
    }
}
