pub mod cmdi;
pub mod idor;
pub mod path_traversal;
pub mod redirect;
pub mod sqli;
pub mod ssrf;
pub mod xss;

pub use crate::config::ProbeRisk;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Probe {
    pub id: String,
    pub payload: String,
    pub oracle: OracleKind,
    pub risk: ProbeRisk,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OracleKind {
    TimingDelta { threshold_ms: u64 },
    CanaryInBody { canary: String },
    CanaryCallback { probe_id: String },
    ErrorPattern { patterns: Vec<String> },
    RedirectToCanary { canary_host: String },
    DifferentialResponse { min_divergence_score: f64 },
    StatusCodeChange { expected_original: u16, trigger_different: bool },
    BodySizeDelta { min_delta_bytes: usize },
}

pub struct ProbeTemplate {
    pub category: &'static str,
    pub probes: Vec<Probe>,
    pub baseline_payload: &'static str,
}

pub fn template_for_category(category: &str, canary_host: &str) -> Option<ProbeTemplate> {
    match category {
        "cmdi" => Some(cmdi::template()),
        "sqli" => Some(sqli::template()),
        "ssrf" => Some(ssrf::template(canary_host)),
        "redirect" => Some(redirect::template(canary_host)),
        "path_traversal" => Some(path_traversal::template()),
        "xss" => Some(xss::template()),
        "idor" => Some(idor::template()),
        _ => None,
    }
}

pub fn category_from_rule_id(rule_id: &str) -> &str {
    rule_id.split('_').nth(1).unwrap_or("unknown")
}

pub fn motif_for_advisory(adv: &frensense::Advisory) -> &'static str {
    match category_from_rule_id(&adv.rule_id) {
        "cmdi" => "CommandExecutionSink",
        "sqli" => "SqlSink",
        "ssrf" => "HttpOutboundSink",
        "redirect" => "HttpResponseSink",
        "path_traversal" => "FileReadSink",
        "xss" => "EvalSink",
        _ => "Unknown",
    }
}
