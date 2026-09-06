use crate::oracle::{OracleEvidence, Verdict};
use crate::probes::Probe;
use crate::tracer::BehavioralTrace;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RuntimeAdvisory {
    pub static_advisory: frensense::Advisory,
    pub status: ConfirmationStatus,
    pub probes_attempted: Vec<ProbeResult>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ConfirmationStatus {
    Confirmed {
        confidence: f64,
        evidence: OracleEvidence,
        confirming_probe: Probe,
    },
    Unconfirmed,
    SanitizationDetected,
    Inconclusive {
        reason: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProbeResult {
    pub probe: Probe,
    pub trace: BehavioralTrace,
    pub verdict: Verdict,
}

impl RuntimeAdvisory {
    pub fn is_confirmed(&self) -> bool {
        matches!(self.status, ConfirmationStatus::Confirmed { .. })
    }

    pub fn combined_confidence(&self) -> f64 {
        if let ConfirmationStatus::Confirmed { confidence, .. } = &self.status {
            (self.static_advisory.confidence * confidence).sqrt()
        } else {
            0.0
        }
    }

    pub fn format_report(&self) -> String {
        let static_adv = &self.static_advisory;
        match &self.status {
            ConfirmationStatus::Confirmed {
                confidence,
                evidence,
                confirming_probe,
            } => format!(
                "[CONFIRMED] {rule} — {file}:{line}\n\
                 Static confidence:  {sc:.0}%\n\
                 Runtime confidence: {rc:.0}%\n\
                 Combined:           {cc:.0}%\n\n\
                 Oracle: {oracle_kind}\n\
                 Evidence: {detail}\n\
                 Probe payload: {payload}\n\n\
                 {observation}\n\
                 Impact: {impact}\n\
                 Fix: {fix}",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
                sc = static_adv.confidence * 100.0,
                rc = confidence * 100.0,
                cc = self.combined_confidence() * 100.0,
                oracle_kind = evidence.oracle_kind,
                detail = evidence.detail,
                payload = confirming_probe.payload,
                observation = static_adv.observation,
                impact = static_adv.impact,
                fix = static_adv.improvement,
            ),
            ConfirmationStatus::SanitizationDetected => format!(
                "[SANITIZED] {rule} — {file}:{line}\n\
                 Static found suspicious code; runtime probes showed input is sanitized.\n\
                 This may be a false positive from the static pass.",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
            ),
            ConfirmationStatus::Unconfirmed => format!(
                "[UNCONFIRMED] {rule} — {file}:{line}\n\
                 {n} probes attempted. No oracle fired.\n\
                 The vulnerability may require authentication, specific state, or\n\
                 a different injection vector not covered by the probe library.",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
                n = self.probes_attempted.len(),
            ),
            ConfirmationStatus::Inconclusive { reason } => format!(
                "[INCONCLUSIVE] {rule} — {file}:{line}\n\
                 {reason}",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
                reason = reason,
            ),
        }
    }
}

impl RuntimeAdvisory {
    pub fn inconclusive(static_advisory: frensense::Advisory, reason: &str) -> Self {
        Self {
            static_advisory,
            status: ConfirmationStatus::Inconclusive {
                reason: reason.to_string(),
            },
            probes_attempted: Vec::new(),
        }
    }
}
