#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub base_url: String,
    pub canary_bind: String,
    pub max_risk: ProbeRisk,
    pub inter_probe_delay_ms: u64,
    pub max_probes_per_endpoint: usize,
    pub max_endpoints_per_session: usize,
    pub auth_header: Option<String>,
    pub output_path: Option<String>,
    pub destructive_probes: bool,
    pub no_limit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum ProbeRisk {
    Safe,
    Low,
    Medium,
    Destructive,
}

impl std::str::FromStr for ProbeRisk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "safe" => Ok(Self::Safe),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "destructive" => Ok(Self::Destructive),
            _ => Err(format!("Unknown risk level: {s}. Use: safe, low, medium, destructive")),
        }
    }
}

impl RuntimeConfig {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            canary_bind: "0.0.0.0:9999".to_string(),
            max_risk: ProbeRisk::Safe,
            inter_probe_delay_ms: 500,
            max_probes_per_endpoint: 10,
            max_endpoints_per_session: 30,
            auth_header: None,
            output_path: None,
            destructive_probes: false,
            no_limit: false,
        }
    }
}
