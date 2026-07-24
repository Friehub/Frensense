use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use clap::Parser;
use tracing_subscriber::EnvFilter;

use frensense_runtime::adapters::detector::detect_framework;
use frensense_runtime::adapters::AuthConvention;
use frensense_runtime::advisory::RuntimeAdvisory;
use frensense_runtime::canary::CanaryServer;
use frensense_runtime::config::{ProbeRisk, RuntimeConfig};
use frensense_runtime::probes::category_from_rule_id;
use frensense_runtime::route_extractor::{match_finding_to_route, RouteBinding};
use frensense_runtime::scheduler::{
    probe_concurrency_degradation, run_probe_campaign, strategy_for_rule_id,
    ConcurrencyVerdict, ProbeStrategy,
};
use frensense_runtime::session::SessionManager;

#[derive(Parser, Debug)]
#[clap(name = "frensense-runtime", about = "Corpus-driven runtime verification for Frensense static findings")]
struct Cli {
    #[clap(long, short, help = "Path to frensense static report JSON")]
    report: PathBuf,

    #[clap(long, short, help = "Target base URL (e.g. http://localhost:3000)")]
    target: String,

    #[clap(long, help = "Canary server bind address (default: 0.0.0.0:9999)", default_value = "0.0.0.0:9999")]
    canary_host: String,

    #[clap(long, help = "Maximum probe risk level (safe, low, medium, destructive)", default_value = "safe")]
    max_risk: ProbeRisk,

    #[clap(long, help = "Authorization header to include in requests")]
    auth_header: Option<String>,

    #[clap(long, help = "Delay between probes in milliseconds", default_value = "500")]
    inter_probe_delay: u64,

    #[clap(long, help = "Output path for runtime report JSON")]
    output: Option<String>,

    #[clap(long, help = "Enable destructive probes (requires confirmation)", default_value_t = false)]
    destructive: bool,

    #[clap(long, help = "Disable endpoint/probe limits", default_value_t = false)]
    no_limit: bool,

    #[clap(long, help = "Project root directory for framework auto-detection")]
    project_root: Option<PathBuf>,

    #[clap(long, help = "Login URL for session acquisition (e.g. http://localhost:3000/login)")]
    login_url: Option<String>,

    #[clap(long, help = "Username for login")]
    auth_username: Option<String>,

    #[clap(long, help = "Password for login")]
    auth_password: Option<String>,

    #[clap(long, help = "Login form username field name (default: userName)", default_value = "userName")]
    login_username_field: String,

    #[clap(long, help = "Login form password field name (default: password)", default_value = "password")]
    login_password_field: String,

    #[clap(long, help = "CSRF form field name (default: _csrf)", default_value = "_csrf")]
    csrf_field: String,

    #[clap(long, help = "Session cookie name (default: connect.sid)", default_value = "connect.sid")]
    session_cookie: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let report_content = tokio::fs::read_to_string(&cli.report)
        .await
        .expect("Failed to read report file");
    let findings: Vec<frensense::Advisory> = serde_json::from_str(&report_content)
        .expect("Failed to parse report JSON");

    let canary_addr: SocketAddr = cli
        .canary_host
        .parse()
        .expect("Invalid canary bind address");
    let canary_server = CanaryServer::new(canary_addr);
    canary_server.start().await;

    let config = RuntimeConfig {
        base_url: cli.target,
        canary_bind: cli.canary_host,
        max_risk: cli.max_risk,
        inter_probe_delay_ms: cli.inter_probe_delay,
        max_probes_per_endpoint: 10,
        max_endpoints_per_session: 30,
        auth_header: cli.auth_header.clone(),
        output_path: cli.output.clone(),
        destructive_probes: cli.destructive,
        no_limit: cli.no_limit,
    };

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    // Acquire session if login URL and credentials are provided
    let session = if let (Some(login_url), Some(username), Some(password)) =
        (&cli.login_url, &cli.auth_username, &cli.auth_password)
    {
        let sm = SessionManager::new(login_url)
            .with_credentials(&cli.login_username_field, &cli.login_password_field)
            .with_csrf(&cli.csrf_field, "_csrf")
            .with_session_cookie(&cli.session_cookie);

        match sm.acquire_session(&client, username, password).await {
            Ok(s) => {
                tracing::info!("Session acquired ({} cookies)", s.cookies.len());
                Some(s)
            }
            Err(e) => {
                tracing::warn!("Session acquisition failed: {e}. Proceeding without auth.");
                None
            }
        }
    } else {
        None
    };

    let mut runtime_advisories: Vec<RuntimeAdvisory> = Vec::new();

    for advisory in &findings {
        let strategy = strategy_for_rule_id(
            &advisory.rule_id,
            &canary_server.bind_addr.to_string(),
        );

        match strategy {
            ProbeStrategy::Http(template) => {
                let adapter = cli.project_root.as_ref().map(|root| {
                    detect_framework(root, &advisory.rule_id)
                });

                let routes: Vec<RouteBinding> = if let Some(ref adapter) = adapter {
                    adapter.extract_routes(
                        Path::new(&advisory.file_path),
                        &advisory.original_content,
                    )
                } else {
                    Vec::new()
                };

                let route = match_finding_to_route(advisory, &routes);
                let route_binding = route.cloned().unwrap_or_else(|| {
                    let fp = advisory.file_path.trim_end_matches(".ts").trim_end_matches(".js").trim_end_matches(".rs").trim_end_matches(".go");
                    let guessed_path = if fp.contains('/') {
                        format!("/{}", fp)
                    } else {
                        format!("/{}", fp)
                    };
                    let guessed_method = if advisory.rule_id.contains("xss") || advisory.rule_id.contains("sqli") {
                        frensense_runtime::route_extractor::HttpMethod::Get
                    } else {
                        frensense_runtime::route_extractor::HttpMethod::Post
                    };
                    RouteBinding {
                        method: guessed_method,
                        path_pattern: guessed_path,
                        handler_file: advisory.file_path.clone(),
                        handler_function: advisory.enclosing_symbol.clone().unwrap_or_default(),
                        injection_points: Vec::new(),
                        framework: frensense_runtime::route_extractor::Framework::Unknown,
                    }
                });

                // Learn injection points from advisory content — same principle as
                // Frensense static learning sources from the corpus: extract param
                // names from the source code rather than guessing.
                let mut route_binding = route_binding;
                if route_binding.injection_points.is_empty() {
                    let points = adapter.as_ref()
                        .map(|a| a.extract_injection_points(&advisory.original_content))
                        .filter(|p| !p.is_empty())
                        .unwrap_or_else(|| {
                            frensense_runtime::route_extractor::extract_injection_points_from_advisory(advisory)
                        });
                    if !points.is_empty() {
                        route_binding.injection_points = points;
                    }
                }
                // If still empty, leave synthetic fallback in scheduler

                let auth_convention = adapter.as_ref()
                    .map(|a| a.auth_convention())
                    .unwrap_or(AuthConvention::BearerToken);
                let auth = cli.auth_header.as_ref().map(|token| {
                    (&auth_convention, token.as_str())
                });

                tracing::info!(
                    "Probing: {} — {}:{} ({})",
                    advisory.rule_id,
                    advisory.file_path,
                    advisory.line,
                    category_from_rule_id(&advisory.rule_id),
                );

                let result = run_probe_campaign(
                    advisory,
                    &route_binding,
                    &template,
                    &client,
                    &canary_server,
                    &config,
                    auth,
                )
                .await;

                if result.is_confirmed() {
                    tracing::info!(
                        "[CONFIRMED] {} — combined confidence: {:.1}%",
                        advisory.rule_id,
                        result.combined_confidence() * 100.0
                    );
                }

                println!("{}", result.format_report());
                runtime_advisories.push(result);
            }
            ProbeStrategy::ConcurrentStress(prober) => {
                let url = format!("{}/", config.base_url.trim_end_matches('/'));
                tracing::info!(
                    "Stress testing: {} — {} (concurrency: {}, duration: {}ms)",
                    advisory.rule_id,
                    advisory.file_path,
                    prober.concurrency,
                    prober.duration_ms,
                );

                let verdict = probe_concurrency_degradation(&client, &url, &prober).await;
                match verdict {
                    ConcurrencyVerdict::Confirmed { p50_ms, p99_ms, degradation_ratio } => {
                        tracing::info!(
                            "[CONFIRMED] {} — p50={}ms p99={}ms ratio={:.1}x",
                            advisory.rule_id, p50_ms, p99_ms, degradation_ratio
                        );
                        let adv = format!(
                            "[CONFIRMED] {} — {}:{}\n\
                             Concurrent stress confirmed degradation\n\
                             p50: {}ms, p99: {}ms, ratio: {:.1}x",
                            advisory.rule_id, advisory.file_path, advisory.line,
                            p50_ms, p99_ms, degradation_ratio,
                        );
                        println!("{adv}");
                    }
                    ConcurrencyVerdict::NotConfirmed => {
                        let adv = format!(
                            "[UNCONFIRMED] {} — {}:{}\n\
                             Concurrent stress: no significant degradation detected",
                            advisory.rule_id, advisory.file_path, advisory.line,
                        );
                        println!("{adv}");
                    }
                    ConcurrencyVerdict::Error(e) => {
                        let adv = format!(
                            "[INCONCLUSIVE] {} — {}:{}\n{}",
                            advisory.rule_id, advisory.file_path, advisory.line, e,
                        );
                        println!("{adv}");
                    }
                }
                runtime_advisories.push(RuntimeAdvisory::inconclusive(
                    advisory.clone(),
                    "Concurrent stress probing applied (see output above)",
                ));
            }
            ProbeStrategy::CannotProbeAtRuntime { reason } => {
                tracing::info!(
                    "[SKIP] {} — {}:{} — {}",
                    advisory.rule_id,
                    advisory.file_path,
                    advisory.line,
                    reason
                );
                runtime_advisories.push(RuntimeAdvisory::inconclusive(
                    advisory.clone(),
                    reason,
                ));
            }
        }

        println!("---");
    }

    if let Some(output_path) = &config.output_path {
        let json = serde_json::to_string_pretty(&runtime_advisories)
            .expect("Failed to serialize runtime advisories");
        tokio::fs::write(output_path, json)
            .await
            .expect("Failed to write output report");
    }
}
