use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let canary = format!(
        "FRENSENSE_{}",
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    );
    ProbeTemplate {
        category: "cmdi",
        baseline_payload: "hello",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("echo {canary}"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "Command substitution canary — output appears in response".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "| sleep 5".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Shell sleep via pipe — timing confirms exec".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "; sleep 5 #".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Shell sleep via semicolon — timing confirms exec".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("& echo {canary}"),
                oracle: OracleKind::CanaryInBody {
                    canary: canary.clone(),
                },
                risk: ProbeRisk::Safe,
                description: "Windows CMD canary — output appears in response".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "& timeout 5".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Windows CMD timeout — timing confirms exec".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "; invalid_cmd_frensense_probe #".to_string(),
                oracle: OracleKind::ErrorPattern {
                    patterns: vec![
                        "command not found".to_string(),
                        "not recognized as".to_string(),
                        "ENOENT".to_string(),
                        "spawn".to_string(),
                    ],
                },
                risk: ProbeRisk::Safe,
                description: "Invalid command — shell error leaks in response".to_string(),
            },
        ],
    }
}
