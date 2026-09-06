use super::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let _canary = format!(
        "FRENSENSE_{}",
        &Uuid::new_v4().to_string()[..8].to_uppercase()
    );
    ProbeTemplate {
        category: "sqli",
        baseline_payload: "test",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'".to_string(),
                oracle: OracleKind::ErrorPattern {
                    patterns: vec![
                        "syntax error at".to_string(),
                        "unterminated quoted string".to_string(),
                        "You have an error in your SQL syntax".to_string(),
                        "mysql_fetch".to_string(),
                        "SQLiteException".to_string(),
                        "near \"'\": syntax error".to_string(),
                        "Unclosed quotation mark".to_string(),
                        "Incorrect syntax near".to_string(),
                        "ORA-".to_string(),
                        "JDBC".to_string(),
                        "pg_query".to_string(),
                    ],
                },
                risk: ProbeRisk::Safe,
                description: "SQL single-quote — triggers syntax error on unparameterized query"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "' OR '1'='1' --".to_string(),
                oracle: OracleKind::DifferentialResponse {
                    min_divergence_score: 0.3,
                },
                risk: ProbeRisk::Safe,
                description: "Boolean injection — changes result set (compare to baseline)"
                    .to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'; SELECT pg_sleep(5)--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "PostgreSQL sleep injection — timing confirms injection".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "' AND SLEEP(5)--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "MySQL sleep injection — timing confirms injection".to_string(),
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'; WAITFOR DELAY '0:0:5'--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "MSSQL wait-for injection — timing confirms injection".to_string(),
            },
        ],
    }
}
