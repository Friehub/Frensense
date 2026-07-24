pub mod adapters;
pub mod advisory;
pub mod canary;
pub mod config;
pub mod oracle;
pub mod probes;
pub mod route_extractor;
pub mod scheduler;
pub mod tracer;

pub use advisory::{ConfirmationStatus, RuntimeAdvisory};
pub use canary::CanaryServer;
pub use config::RuntimeConfig;
pub use oracle::{evaluate_oracle, OracleEvidence, Verdict};
pub use probes::{OracleKind, Probe, ProbeRisk, ProbeTemplate};
pub use route_extractor::{
    extract_injection_points_from_advisory, extract_routes, match_finding_to_route, Framework,
    HttpMethod, InjectionPoint, ParameterLocation, RouteBinding,
};
pub use scheduler::{probe_concurrency_degradation, run_probe_campaign, ConcurrentStressProber, ProbeStrategy};
pub use tracer::{execute_probe, BehavioralTrace};
