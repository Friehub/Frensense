use crate::probes::OracleKind;
use crate::tracer::BehavioralTrace;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Verdict {
    Confirmed {
        confidence: f64,
        evidence: OracleEvidence,
    },
    NotConfirmed,
    SanitizationDetected,
    Inconclusive {
        reason: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OracleEvidence {
    pub oracle_kind: String,
    pub detail: String,
    pub raw_value: String,
}

pub fn evaluate_oracle(
    oracle: &OracleKind,
    probe_trace: &BehavioralTrace,
    baseline_trace: &BehavioralTrace,
) -> Verdict {
    match oracle {
        OracleKind::TimingDelta { threshold_ms } => {
            let delta = probe_trace
                .duration_ms
                .saturating_sub(baseline_trace.duration_ms);
            if delta >= *threshold_ms {
                Verdict::Confirmed {
                    confidence: 0.82,
                    evidence: OracleEvidence {
                        oracle_kind: "timing_delta".to_string(),
                        detail: format!(
                            "Response delayed {delta}ms above baseline (threshold: {threshold_ms}ms)"
                        ),
                        raw_value: delta.to_string(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::CanaryInBody { canary } => {
            let body = String::from_utf8_lossy(&probe_trace.response_body);
            if body.contains(canary.as_str()) {
                Verdict::Confirmed {
                    confidence: 0.97,
                    evidence: OracleEvidence {
                        oracle_kind: "canary_in_body".to_string(),
                        detail: format!("Canary string '{canary}' found in response body"),
                        raw_value: canary.clone(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::CanaryCallback { probe_id } => {
            if probe_trace.canary_received {
                Verdict::Confirmed {
                    confidence: 0.99,
                    evidence: OracleEvidence {
                        oracle_kind: "canary_callback".to_string(),
                        detail: format!(
                            "Inbound connection received at canary server for probe {probe_id}"
                        ),
                        raw_value: probe_id.clone(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::ErrorPattern { patterns } => {
            let body = String::from_utf8_lossy(&probe_trace.response_body).to_lowercase();
            for pat in patterns {
                if body.contains(pat.to_lowercase().as_str()) {
                    return Verdict::Confirmed {
                        confidence: 0.78,
                        evidence: OracleEvidence {
                            oracle_kind: "error_pattern".to_string(),
                            detail: format!("Error pattern '{pat}' found in response"),
                            raw_value: pat.clone(),
                        },
                    };
                }
            }
            Verdict::NotConfirmed
        }

        OracleKind::RedirectToCanary { canary_host } => {
            if let Some(loc) = &probe_trace.redirect_location {
                if loc.contains(canary_host.as_str()) {
                    return Verdict::Confirmed {
                        confidence: 0.95,
                        evidence: OracleEvidence {
                            oracle_kind: "redirect_to_canary".to_string(),
                            detail: format!("Location header redirects to canary host: {loc}"),
                            raw_value: loc.clone(),
                        },
                    };
                }
            }
            Verdict::NotConfirmed
        }

        OracleKind::DifferentialResponse {
            min_divergence_score,
        } => {
            let div = probe_trace.divergence_from(baseline_trace);
            if div >= *min_divergence_score {
                Verdict::Confirmed {
                    confidence: 0.60 + (div - min_divergence_score) * 0.3,
                    evidence: OracleEvidence {
                        oracle_kind: "differential_response".to_string(),
                        detail: format!(
                            "Response divergence {div:.2} exceeds threshold {min_divergence_score:.2}"
                        ),
                        raw_value: format!("{div:.4}"),
                    },
                }
            } else if div < 0.05 {
                Verdict::SanitizationDetected
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::StatusCodeChange {
            expected_original,
            trigger_different,
        } => {
            let changed = probe_trace.status_code != *expected_original;
            if changed == *trigger_different {
                Verdict::Confirmed {
                    confidence: 0.65,
                    evidence: OracleEvidence {
                        oracle_kind: "status_code_change".to_string(),
                        detail: format!(
                            "Status changed from {} to {}",
                            expected_original, probe_trace.status_code
                        ),
                        raw_value: probe_trace.status_code.to_string(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::BodySizeDelta { min_delta_bytes } => {
            let delta = (probe_trace.response_size_bytes as i64
                - baseline_trace.response_size_bytes as i64)
                .unsigned_abs() as usize;
            if delta >= *min_delta_bytes {
                Verdict::Confirmed {
                    confidence: 0.55,
                    evidence: OracleEvidence {
                        oracle_kind: "body_size_delta".to_string(),
                        detail: format!(
                            "Body size changed by {delta} bytes (threshold: {min_delta_bytes})"
                        ),
                        raw_value: delta.to_string(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }
    }
}
