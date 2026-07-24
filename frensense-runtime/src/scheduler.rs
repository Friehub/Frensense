use std::time::Duration;

use crate::adapters::AuthConvention;
use crate::advisory::{ConfirmationStatus, ProbeResult, RuntimeAdvisory};
use crate::canary::CanaryServer;
use crate::config::RuntimeConfig;
use crate::oracle::{evaluate_oracle, Verdict};
use crate::probes::{category_from_rule_id, template_for_category, Probe, ProbeRisk, ProbeTemplate};
use crate::route_extractor::{InjectionPoint, RouteBinding};
use crate::tracer::{execute_probe, ProbeTarget};

pub enum ProbeStrategy {
    Http(ProbeTemplate),
    ConcurrentStress(ConcurrentStressProber),
    CannotProbeAtRuntime { reason: &'static str },
}

pub struct ConcurrentStressProber {
    pub concurrency: usize,
    pub duration_ms: u64,
    pub expect_degradation: bool,
}

pub enum ConcurrencyVerdict {
    Confirmed { p50_ms: u64, p99_ms: u64, degradation_ratio: f64 },
    NotConfirmed,
    Error(String),
}

pub fn strategy_for_rule_id(rule_id: &str, canary_host: &str) -> ProbeStrategy {
    let category = category_from_rule_id(rule_id);
    match category {
        "cmdi" | "sqli" | "ssrf" | "redirect" | "path_traversal" | "xss" | "idor" => {
            if let Some(template) = template_for_category(category, canary_host) {
                ProbeStrategy::Http(template)
            } else {
                ProbeStrategy::CannotProbeAtRuntime {
                    reason: "No probe template available for this category",
                }
            }
        }
        "auth" | "cors" | "csrf" => ProbeStrategy::CannotProbeAtRuntime {
            reason: "Auth/CORS/CSRF probing requires multi-session context (planned)",
        },
        _ if rule_id.starts_with("rust_async") || rule_id.starts_with("rust_race") => {
            ProbeStrategy::ConcurrentStress(ConcurrentStressProber {
                concurrency: 50,
                duration_ms: 3000,
                expect_degradation: true,
            })
        }
        _ if rule_id.starts_with("tsx_use") => {
            ProbeStrategy::CannotProbeAtRuntime {
                reason: "React hook bugs require headless browser probing (planned)",
            }
        }
        _ => ProbeStrategy::CannotProbeAtRuntime {
            reason: "No HTTP surface; static analysis only",
        },
    }
}

pub async fn run_probe_campaign(
    static_advisory: &frensense::Advisory,
    route: &RouteBinding,
    template: &ProbeTemplate,
    client: &reqwest::Client,
    canary_server: &CanaryServer,
    config: &RuntimeConfig,
    auth: Option<(&AuthConvention, &str)>,
) -> RuntimeAdvisory {
    let default_point = InjectionPoint {
        location: crate::route_extractor::ParameterLocation::Body,
        name: "input".to_string(),
        taint_origin: None,
    };
    let injection_point = route.injection_points.first().unwrap_or(&default_point);

    let baseline = execute_probe(
        client,
        &ProbeTarget {
            route,
            base_url: &config.base_url,
            injection_point,
            auth,
        },
        &Probe {
            id: "baseline".to_string(),
            payload: template.baseline_payload.to_string(),
            oracle: crate::probes::OracleKind::DifferentialResponse {
                min_divergence_score: 99.0,
            },
            risk: ProbeRisk::Safe,
            description: "Baseline — benign input to establish normal response".to_string(),
        },
        canary_server,
    )
    .await;

    if baseline.status_code == 0 {
        return RuntimeAdvisory::inconclusive(
            static_advisory.clone(),
            "Endpoint unreachable",
        );
    }

    let mut probe_results: Vec<ProbeResult> = Vec::new();

    for probe in &template.probes {
        if probe.risk > config.max_risk {
            continue;
        }

        let trace = execute_probe(
            client,
            &ProbeTarget {
                route,
                base_url: &config.base_url,
                injection_point,
                auth,
            },
            probe,
            canary_server,
        )
        .await;

        let verdict = evaluate_oracle(&probe.oracle, &trace, &baseline);
        let is_confirmed = matches!(verdict, Verdict::Confirmed { .. });
        let is_sanitized = matches!(verdict, Verdict::SanitizationDetected);

        probe_results.push(ProbeResult {
            probe: probe.clone(),
            trace,
            verdict: verdict.clone(),
        });

        if is_confirmed {
            break;
        }
        if is_sanitized {
            break;
        }

        tokio::time::sleep(Duration::from_millis(config.inter_probe_delay_ms)).await;
    }

    build_runtime_advisory(static_advisory.clone(), probe_results)
}

pub async fn probe_concurrency_degradation(
    client: &reqwest::Client,
    url: &str,
    prober: &ConcurrentStressProber,
) -> ConcurrencyVerdict {
    let start = std::time::Instant::now();
    let mut latencies: Vec<u64> = Vec::new();
    let deadline = Duration::from_millis(prober.duration_ms);

    loop {
        if start.elapsed() >= deadline {
            break;
        }

        let mut handles = Vec::new();
        for _ in 0..prober.concurrency {
            let client = client.clone();
            let url = url.to_string();
            handles.push(tokio::spawn(async move {
                let req_start = std::time::Instant::now();
                match client.get(&url).send().await {
                    Ok(_) => Some(req_start.elapsed().as_millis() as u64),
                    Err(_) => None,
                }
            }));
        }

        for handle in handles {
            if let Ok(Some(lat)) = handle.await {
                latencies.push(lat);
            }
        }
    }

    if latencies.is_empty() {
        return ConcurrencyVerdict::Error("No successful requests".to_string());
    }

    latencies.sort_unstable();
    let p50 = latencies[latencies.len() / 2];
    let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
    let ratio = if p50 > 0 { p99 as f64 / p50 as f64 } else { 1.0 };

    if ratio > 10.0 {
        ConcurrencyVerdict::Confirmed {
            p50_ms: p50,
            p99_ms: p99,
            degradation_ratio: ratio,
        }
    } else {
        ConcurrencyVerdict::NotConfirmed
    }
}

fn build_runtime_advisory(
    static_advisory: frensense::Advisory,
    results: Vec<ProbeResult>,
) -> RuntimeAdvisory {
    let confirmed = results.iter().find_map(|r| {
        if let Verdict::Confirmed {
            confidence,
            evidence,
        } = &r.verdict
        {
            Some((*confidence, evidence.clone(), r.probe.clone()))
        } else {
            None
        }
    });

    let sanitization_detected =
        results
            .iter()
            .any(|r| matches!(r.verdict, Verdict::SanitizationDetected));

    let status = if let Some((confidence, evidence, probe)) = confirmed {
        ConfirmationStatus::Confirmed {
            confidence,
            evidence,
            confirming_probe: probe,
        }
    } else if sanitization_detected {
        ConfirmationStatus::SanitizationDetected
    } else if results.is_empty() {
        ConfirmationStatus::Inconclusive {
            reason: "No applicable probes for this category".to_string(),
        }
    } else {
        ConfirmationStatus::Unconfirmed
    };

    RuntimeAdvisory {
        static_advisory,
        status,
        probes_attempted: results,
    }
}
