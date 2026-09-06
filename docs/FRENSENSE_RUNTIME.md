# Frensense Runtime — Architecture and Implementation Guide

A corpus-driven, deterministic runtime verification layer that turns every
Frensense static finding into a live probe, then confirms or refutes it by
observing real application behaviour.

No LLM. No guessing. Probes are derived from the same corpus that drives
static analysis.

---

## The Core Idea

Frensense Static knows what **buggy code looks like**.
Frensense Runtime knows what **buggy behaviour looks like**.

The relationship is direct:

```
Static finding (ts_cmdi_exec_direct, file: routes/run.ts, line 42)
    │
    ├─ corpus positive tells us:  exec(req.body.cmd)  → CommandExecutionSink
    ├─ taint analysis tells us:   req.body.cmd → cmd → exec(cmd)
    ├─ route scanner tells us:    POST /api/run, body param: cmd
    │
    └─► Runtime generates:        POST /api/run  {"cmd": "; sleep 5 #"}
                                  Oracle: response_time > 5000ms → CONFIRMED
```

Every runtime probe is scoped to exactly the endpoint, parameter, and
vulnerability class that the static pass already suspects. This is fundamentally
different from a generic fuzzer, which tries everything everywhere. Frensense
Runtime has a precise target list before it sends a single byte.

---

## Concept Mapping — Static → Runtime

| Frensense Static | Frensense Runtime | Purpose |
|---|---|---|
| `FunctionFingerprint` | `BehavioralTrace` | The unit of comparison |
| Corpus positive (buggy code) | `ProbeTemplate` | What to send |
| Corpus negative (safe code) | Baseline capture | Normal response to compare against |
| `SemanticFilter.contains_call_to` | `ProbeSelector.required_sink_motif` | Gate: only probe if relevant |
| LSH pre-filter | Route pre-filter | Only probe endpoints with static findings |
| API IDF gate | Probe priority order | Highest-confidence probes run first |
| `compute_similarity` score | Oracle confidence score | How sure are we? |
| `Advisory` | `RuntimeAdvisory` | Final output |
| Composition layer | Confirmation aggregator | Multiple probe types → higher confidence |
| `TaintOrigin::UserInput` | Injection point selection | Which HTTP field to inject |
| `DataFlowEngine` | Canary listener | Confirms taint reached the sink |
| `observation / impact` | Evidence block in report | Why we flagged it |

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frensense Runtime                         │
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   Static     │    │    Route     │    │   Canary     │  │
│  │  Report JSON │───►│  Extractor   │    │   Server     │  │
│  └──────────────┘    └──────┬───────┘    └──────┬───────┘  │
│                             │                   │           │
│                      ┌──────▼───────┐           │           │
│                      │   Probe      │           │           │
│                      │  Scheduler   │           │           │
│                      └──────┬───────┘           │           │
│                             │                   │           │
│                      ┌──────▼───────┐           │           │
│                      │   Probe      │◄──────────┘           │
│                      │  Executor    │  (waits for callbacks) │
│                      └──────┬───────┘                       │
│                             │                               │
│                      ┌──────▼───────┐                       │
│                      │  Behavioral  │                       │
│                      │    Tracer    │                       │
│                      └──────┬───────┘                       │
│                             │                               │
│                      ┌──────▼───────┐                       │
│                      │   Oracle     │                       │
│                      │  Evaluator   │                       │
│                      └──────┬───────┘                       │
│                             │                               │
│                      ┌──────▼───────┐                       │
│                      │  Runtime     │                       │
│                      │  Reporter    │                       │
│                      └──────────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Phase 1 — Route Extraction

The static `Advisory` knows the file path, function name, line number, and
which HTTP parameters are tainted (`req.body.cmd`, `req.query.url`, etc.).
The route extractor maps this to an actual HTTP endpoint.

### New file: `frensense-runtime/src/route_extractor.rs`

```rust
// SPDX-License-Identifier: MIT
//! Lightweight route extractor.
//!
//! Scans source files for framework route registration patterns using the
//! same tree-sitter infrastructure as Frensense Static. Does NOT need to
//! run the application. Pure static AST pass on the same files.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteBinding {
    /// HTTP method: GET, POST, PUT, DELETE, PATCH, ALL
    pub method: HttpMethod,
    /// Path pattern as declared: "/api/users/:id"
    pub path_pattern: String,
    /// Source file where the route is registered
    pub handler_file: String,
    /// Function name of the handler (matches Advisory.enclosing_symbol)
    pub handler_function: String,
    /// HTTP parameters that feed user input into this handler
    pub injection_points: Vec<InjectionPoint>,
    /// Framework that registered this route
    pub framework: Framework,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InjectionPoint {
    /// Where in the HTTP request this comes from
    pub location: ParameterLocation,
    /// Parameter name, e.g. "cmd" from req.body.cmd
    pub name: String,
    /// What the static analysis knows flows from this point
    pub taint_origin: Option<String>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ParameterLocation {
    Body,       // req.body.x
    Query,      // req.query.x / req.params.x (URL query string)
    PathParam,  // req.params.x (URL path segment :x)
    Header,     // req.headers.x
    Cookie,     // req.cookies.x
    FormData,   // multipart form
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum HttpMethod { Get, Post, Put, Delete, Patch, All }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Framework {
    Express,    // app.get / router.post / app.use
    Fastify,    // fastify.get / fastify.route
    Koa,        // router.get / ctx.body
    NestJs,     // @Get() @Post() decorators
    GoNetHttp,  // http.HandleFunc / mux.HandleFunc
    GoGin,      // r.GET / r.POST
    GoEcho,     // e.GET / e.POST
    AxumRust,   // .route("/path", get(handler))
    Unknown,
}
```

### Route extraction logic

For each file containing a static finding, run a second tree-sitter pass
looking specifically for route registration patterns. This is a separate,
focused AST walk — NOT the full fingerprinting pass.

```rust
pub fn extract_routes(file_path: &Path, source: &str, lang: Language) -> Vec<RouteBinding> {
    let tree = parse(source, lang);
    let mut routes = Vec::new();
    extract_routes_recursive(tree.root_node(), source, &mut routes, file_path);
    routes
}

fn extract_routes_recursive(
    node: Node<'_>,
    source: &str,
    routes: &mut Vec<RouteBinding>,
    file: &Path,
) {
    // Express: app.get("/path", handler) or router.post("/path", async (req, res) => { ... })
    if node.kind() == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            let callee = &source[func.start_byte()..func.end_byte()];
            if let Some((method, framework)) = detect_route_registration(callee) {
                if let Some(binding) = extract_express_route(node, source, file, method, framework) {
                    routes.push(binding);
                    return; // don't recurse into route args — the handler is already captured
                }
            }
        }
    }
    // Recurse
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_routes_recursive(cursor.node(), source, routes, file);
            if !cursor.goto_next_sibling() { break; }
        }
    }
}

/// Returns (HttpMethod, Framework) if the callee looks like a route registration.
fn detect_route_registration(callee: &str) -> Option<(HttpMethod, Framework)> {
    // Express / Koa: app.get, router.post, app.all, etc.
    let segments: Vec<&str> = callee.splitn(2, '.').collect();
    if segments.len() == 2 {
        let method = match segments[1] {
            "get"    => Some(HttpMethod::Get),
            "post"   => Some(HttpMethod::Post),
            "put"    => Some(HttpMethod::Put),
            "delete" => Some(HttpMethod::Delete),
            "patch"  => Some(HttpMethod::Patch),
            "all"    => Some(HttpMethod::All),
            "route"  => Some(HttpMethod::Post), // Fastify .route({method:...})
            _ => None,
        };
        if let Some(m) = method {
            let fw = detect_framework(segments[0]);
            return Some((m, fw));
        }
    }
    // Go: http.HandleFunc, mux.HandleFunc
    if callee == "http.HandleFunc" || callee.ends_with(".HandleFunc") {
        return Some((HttpMethod::All, Framework::GoNetHttp));
    }
    None
}

fn detect_framework(receiver: &str) -> Framework {
    match receiver {
        "app" | "router" | "express" => Framework::Express,
        "fastify" | "server"         => Framework::Fastify,
        "r" | "engine"               => Framework::GoGin,
        "e"                          => Framework::GoEcho,
        _                            => Framework::Unknown,
    }
}
```

### Matching routes to findings

```rust
pub fn match_finding_to_route<'a>(
    advisory: &Advisory,
    routes: &'a [RouteBinding],
) -> Option<&'a RouteBinding> {
    routes.iter().find(|r| {
        // Primary match: handler file + function name
        let file_match = r.handler_file.ends_with(&advisory.file_path)
            || advisory.file_path.ends_with(&r.handler_file);
        let fn_match = advisory.enclosing_symbol.as_deref()
            .map_or(false, |sym| sym == r.handler_function || r.handler_function.contains(sym));
        file_match && fn_match
    })
    // Fallback: just file match if function name not resolved
    .or_else(|| {
        routes.iter().find(|r| {
            r.handler_file.ends_with(&advisory.file_path)
        })
    })
}
```

### Extracting injection points from Advisory

The `Advisory` carries `original_content` (the matched function body) and
`taint_branch_ratio`. We extract parameter references from it:

```rust
pub fn extract_injection_points_from_advisory(advisory: &Advisory) -> Vec<InjectionPoint> {
    let mut points = Vec::new();
    let content = &advisory.original_content;

    // Patterns: req.body.X, req.query.X, req.params.X, req.headers.X, ctx.request.body.X
    let patterns: &[(&str, ParameterLocation)] = &[
        ("req.body.",     ParameterLocation::Body),
        ("req.query.",    ParameterLocation::Query),
        ("req.params.",   ParameterLocation::PathParam),
        ("req.headers.",  ParameterLocation::Header),
        ("req.cookies.",  ParameterLocation::Cookie),
        ("ctx.request.",  ParameterLocation::Body),
        ("r.URL.Query()", ParameterLocation::Query),    // Go net/http
        ("r.FormValue(",  ParameterLocation::FormData), // Go net/http
        ("c.Query(",      ParameterLocation::Query),    // Go Gin
        ("c.PostForm(",   ParameterLocation::Body),     // Go Gin
    ];

    for (prefix, location) in patterns {
        let mut rest = content.as_str();
        while let Some(pos) = rest.find(prefix) {
            let after = &rest[pos + prefix.len()..];
            // Extract the parameter name up to the first non-identifier character
            let name_end = after.find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after.len());
            let name = after[..name_end].to_string();
            if !name.is_empty() && !points.iter().any(|p: &InjectionPoint| p.name == name) {
                points.push(InjectionPoint {
                    location: *location,
                    name,
                    taint_origin: Some("user_input".to_string()),
                });
            }
            rest = &rest[pos + prefix.len()..];
        }
    }
    points
}
```

---

## Phase 2 — Probe Templates

Probes are stored in a compiled-in library (same concept as the embedded corpus
bundle). Each probe template belongs to a vulnerability category and specifies
payloads plus the oracle that detects success.

### Category derivation from `rule_id`

The `Advisory.rule_id` encodes the category at segment index 1:
`ts_cmdi_exec_direct` → category = `"cmdi"`.

```rust
pub fn category_from_rule_id(rule_id: &str) -> &str {
    rule_id.split('_').nth(1).unwrap_or("unknown")
}
```

### New file: `frensense-runtime/src/probes/mod.rs`

```rust
pub mod cmdi;
pub mod sqli;
pub mod ssrf;
pub mod redirect;
pub mod path_traversal;
pub mod xss;
pub mod idor;

/// A single probe: one payload + one oracle type.
#[derive(Debug, Clone)]
pub struct Probe {
    /// Unique ID for correlating canary callbacks
    pub id: String,
    /// The string value to inject into the parameter
    pub payload: String,
    /// How to interpret the application's response
    pub oracle: OracleKind,
    /// Risk level — determines if --safe-only mode runs this probe
    pub risk: ProbeRisk,
    /// Human description for the report
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProbeRisk {
    /// No side effects: read-only, timing-based, or canary-based
    Safe,
    /// May generate log noise or trigger alerts
    Low,
    /// May create records in the database
    Medium,
    /// Never run without explicit --destructive flag
    Destructive,
}

#[derive(Debug, Clone)]
pub enum OracleKind {
    /// Response time exceeds baseline + threshold
    TimingDelta { threshold_ms: u64 },
    /// A unique string appears in the response body
    CanaryInBody { canary: String },
    /// A request arrives at the canary callback server
    CanaryCallback { probe_id: String },
    /// Response contains a known error message pattern
    ErrorPattern { patterns: Vec<String> },
    /// Location/redirect header contains our canary host
    RedirectToCanary { canary_host: String },
    /// Response differs significantly from baseline
    DifferentialResponse { min_divergence_score: f64 },
    /// HTTP status code changed from baseline
    StatusCodeChange { expected_original: u16, trigger_different: bool },
    /// Response body size differs from baseline by more than N bytes
    BodySizeDelta { min_delta_bytes: usize },
}

/// A set of probes for one vulnerability category.
/// Ordered by reliability: highest-confidence oracle first.
pub struct ProbeTemplate {
    pub category: &'static str,
    pub probes: Vec<Probe>,
    /// A benign payload used to establish the baseline response
    pub baseline_payload: &'static str,
}

pub fn template_for_category(category: &str) -> Option<ProbeTemplate> {
    match category {
        "cmdi"           => Some(cmdi::template()),
        "sqli"           => Some(sqli::template()),
        "ssrf"           => Some(ssrf::template()),
        "redirect"       => Some(redirect::template()),
        "path_traversal" => Some(path_traversal::template()),
        "xss"            => Some(xss::template()),
        "idor"           => Some(idor::template()),
        _                => None,
    }
}
```

### `frensense-runtime/src/probes/cmdi.rs`

```rust
use super::{Probe, ProbeRisk, ProbeTemplate, OracleKind};
use uuid::Uuid;

pub fn template() -> ProbeTemplate {
    let canary = format!("FRENSENSE_{}", &Uuid::new_v4().to_string()[..8].to_uppercase());
    ProbeTemplate {
        category: "cmdi",
        baseline_payload: "hello",
        probes: vec![
            // 1. Canary-based — most reliable, zero side effects
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("; echo {canary} #"),
                oracle: OracleKind::CanaryInBody { canary: canary.clone() },
                risk: ProbeRisk::Safe,
                description: "Shell canary echo — output appears in response",
            },
            // 2. Time-based — works when output is not reflected
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "| sleep 5".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Shell sleep via pipe — timing confirms exec",
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "; sleep 5 #".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Shell sleep via semicolon — timing confirms exec",
            },
            // 3. Windows variants
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("& echo {canary}"),
                oracle: OracleKind::CanaryInBody { canary: canary.clone() },
                risk: ProbeRisk::Safe,
                description: "Windows CMD canary — output appears in response",
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "& timeout 5".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "Windows CMD timeout — timing confirms exec",
            },
            // 4. Error-based — detects shell error reflection
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
                description: "Invalid command — shell error leaks in response",
            },
        ],
    }
}
```

### `frensense-runtime/src/probes/sqli.rs`

```rust
pub fn template() -> ProbeTemplate {
    let canary = format!("FRENSENSE_{}", &Uuid::new_v4().to_string()[..8].to_uppercase());
    ProbeTemplate {
        category: "sqli",
        baseline_payload: "test",
        probes: vec![
            // 1. Error-based — most reliable and fastest
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'".to_string(),
                oracle: OracleKind::ErrorPattern {
                    patterns: vec![
                        // PostgreSQL
                        "syntax error at".to_string(), "unterminated quoted string".to_string(),
                        // MySQL
                        "You have an error in your SQL syntax".to_string(), "mysql_fetch".to_string(),
                        // SQLite
                        "SQLiteException".to_string(), "near \"'\": syntax error".to_string(),
                        // MSSQL
                        "Unclosed quotation mark".to_string(), "Incorrect syntax near".to_string(),
                        // Generic
                        "ORA-".to_string(), "JDBC".to_string(), "pg_query".to_string(),
                    ],
                },
                risk: ProbeRisk::Safe,
                description: "SQL single-quote — triggers syntax error on unparameterized query",
            },
            // 2. Boolean-based differential
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "' OR '1'='1' --".to_string(),
                oracle: OracleKind::DifferentialResponse { min_divergence_score: 0.3 },
                risk: ProbeRisk::Safe,
                description: "Boolean injection — changes result set (compare to baseline)",
            },
            // 3. Time-based — PostgreSQL
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'; SELECT pg_sleep(5)--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "PostgreSQL sleep injection — timing confirms injection",
            },
            // 4. Time-based — MySQL
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "' AND SLEEP(5)--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "MySQL sleep injection — timing confirms injection",
            },
            // 5. Stacked — MSSQL
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "'; WAITFOR DELAY '0:0:5'--".to_string(),
                oracle: OracleKind::TimingDelta { threshold_ms: 4500 },
                risk: ProbeRisk::Safe,
                description: "MSSQL wait-for injection — timing confirms injection",
            },
        ],
    }
}
```

### `frensense-runtime/src/probes/ssrf.rs`

```rust
pub fn template(canary_host: &str) -> ProbeTemplate {
    // The canary host is the IP/hostname of our CanaryServer (Phase 1 listener)
    ProbeTemplate {
        category: "ssrf",
        baseline_payload: "https://example.com",
        probes: vec![
            // 1. Callback-based — definitive
            Probe {
                id: {
                    let id = Uuid::new_v4().to_string();
                    id
                },
                payload: format!("http://{canary_host}/frensense-probe"),
                oracle: OracleKind::CanaryCallback {
                    probe_id: "ssrf_callback".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "SSRF callback to canary server — inbound connection confirms fetch",
            },
            // 2. Cloud metadata — confirms internal network access
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "http://169.254.169.254/latest/meta-data/iam/security-credentials/".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "iam".to_string(), // metadata response always contains "iam"
                },
                risk: ProbeRisk::Safe,
                description: "AWS metadata endpoint — response body confirms internal network access",
            },
            // 3. DNS-rebinding safe variant
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("http://ssrf.{canary_host}"),
                oracle: OracleKind::CanaryCallback {
                    probe_id: "ssrf_dns".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "SSRF via DNS lookup — canary DNS resolution confirms fetch attempt",
            },
        ],
    }
}
```

### `frensense-runtime/src/probes/path_traversal.rs`

```rust
pub fn template() -> ProbeTemplate {
    let canary = format!("FRENSENSE_{}", &Uuid::new_v4().to_string()[..8].to_uppercase());
    ProbeTemplate {
        category: "path_traversal",
        baseline_payload: "profile.txt",
        probes: vec![
            // 1. Read /etc/passwd — definitive on Linux
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "../../../../etc/passwd".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "root:x:0:0".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Path traversal to /etc/passwd — root line confirms file read",
            },
            // 2. URL-encoded variant
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "..%2F..%2F..%2F..%2Fetc%2Fpasswd".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "root:x:0:0".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "URL-encoded traversal — bypasses naive string filters",
            },
            // 3. Windows variant
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: "..\\..\\..\\windows\\win.ini".to_string(),
                oracle: OracleKind::CanaryInBody {
                    canary: "[fonts]".to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Windows path traversal to win.ini",
            },
        ],
    }
}
```

### `frensense-runtime/src/probes/redirect.rs`

```rust
pub fn template(canary_host: &str) -> ProbeTemplate {
    ProbeTemplate {
        category: "redirect",
        baseline_payload: "/dashboard",
        probes: vec![
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("https://{canary_host}/probe"),
                oracle: OracleKind::RedirectToCanary {
                    canary_host: canary_host.to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Open redirect to external host — Location header confirms",
            },
            Probe {
                id: Uuid::new_v4().to_string(),
                payload: format!("//{canary_host}/probe"),
                oracle: OracleKind::RedirectToCanary {
                    canary_host: canary_host.to_string(),
                },
                risk: ProbeRisk::Safe,
                description: "Protocol-relative redirect — bypasses http:// prefix check",
            },
        ],
    }
}
```

---

## Phase 3 — Behavioral Tracer

The `BehavioralTrace` is the runtime equivalent of `FunctionFingerprint`. It
captures every observable dimension of an HTTP response.

### New file: `frensense-runtime/src/tracer.rs`

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// The complete observable state of one HTTP probe execution.
/// This is the runtime equivalent of FunctionFingerprint.
#[derive(Debug, Clone)]
pub struct BehavioralTrace {
    pub probe_id: String,
    pub payload: String,
    pub status_code: u16,
    pub response_body: Vec<u8>,
    pub response_body_hash: u64,
    pub response_size_bytes: usize,
    pub response_headers: HashMap<String, String>,
    pub duration_ms: u64,
    pub canary_received: bool,
    /// Whether the response redirected (Location header set)
    pub redirect_location: Option<String>,
    /// Any error that occurred during the request
    pub transport_error: Option<String>,
    /// Timestamp of the probe
    pub sent_at: std::time::SystemTime,
}

impl BehavioralTrace {
    /// Compute a divergence score between this trace and a baseline.
    /// Returns [0.0, 1.0] — higher means more different.
    /// Analogous to 1.0 - similarity_score in the static engine.
    pub fn divergence_from(&self, baseline: &BehavioralTrace) -> f64 {
        let mut score = 0.0f64;
        let mut weight_total = 0.0f64;

        // Status code change — strong signal
        let status_weight = 0.35;
        if self.status_code != baseline.status_code {
            score += status_weight;
        }
        weight_total += status_weight;

        // Body size change — medium signal
        let size_weight = 0.25;
        let size_delta = (self.response_size_bytes as i64 - baseline.response_size_bytes as i64).unsigned_abs();
        let size_ratio = size_delta as f64 / (baseline.response_size_bytes.max(1) as f64);
        score += size_weight * size_ratio.min(1.0);
        weight_total += size_weight;

        // Body hash change — strong signal
        let body_weight = 0.25;
        if self.response_body_hash != baseline.response_body_hash {
            score += body_weight;
        }
        weight_total += body_weight;

        // Timing change — weak signal (network jitter)
        let timing_weight = 0.15;
        let timing_delta = self.duration_ms.saturating_sub(baseline.duration_ms);
        if timing_delta > 1000 {
            score += timing_weight * (timing_delta as f64 / 5000.0).min(1.0);
        }
        weight_total += timing_weight;

        if weight_total > 0.0 { score / weight_total } else { 0.0 }
    }
}

/// Execute a single HTTP probe and return its behavioral trace.
pub async fn execute_probe(
    client: &reqwest::Client,
    target: &ProbeTarget,
    probe: &Probe,
    canary_server: &CanaryServer,
) -> BehavioralTrace {
    let probe_id = probe.id.clone();
    canary_server.register_pending(&probe_id);

    let request = build_request(client, target, &probe.payload);
    let sent_at = std::time::SystemTime::now();
    let start = Instant::now();

    match request.send().await {
        Ok(resp) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            let status_code = resp.status().as_u16();
            let headers: HashMap<String, String> = resp.headers().iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();
            let redirect_location = headers.get("location").cloned();
            let body = resp.bytes().await.unwrap_or_default().to_vec();
            let body_hash = hash_bytes(&body);

            // Wait up to 500ms for canary callback to arrive
            tokio::time::sleep(Duration::from_millis(500)).await;
            let canary_received = canary_server.check_received(&probe_id);

            BehavioralTrace {
                probe_id,
                payload: probe.payload.clone(),
                status_code,
                response_body: body.clone(),
                response_body_hash: body_hash,
                response_size_bytes: body.len(),
                response_headers: headers,
                duration_ms,
                canary_received,
                redirect_location,
                transport_error: None,
                sent_at,
            }
        }
        Err(e) => BehavioralTrace {
            probe_id,
            payload: probe.payload.clone(),
            status_code: 0,
            response_body: Vec::new(),
            response_body_hash: 0,
            response_size_bytes: 0,
            response_headers: HashMap::new(),
            duration_ms: start.elapsed().as_millis() as u64,
            canary_received: false,
            redirect_location: None,
            transport_error: Some(e.to_string()),
            sent_at,
        },
    }
}
```

---

## Phase 4 — Canary Server

Required for SSRF and some CMDI/XSS probes. A simple TCP listener that records
inbound connections by probe ID.

### New file: `frensense-runtime/src/canary.rs`

```rust
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, Default)]
pub struct CanaryServer {
    pub bind_addr: SocketAddr,
    inner: Arc<Mutex<CanaryState>>,
}

#[derive(Default, Debug)]
struct CanaryState {
    pending: HashMap<String, ()>,
    received: HashMap<String, ReceivedCallback>,
}

#[derive(Debug, Clone)]
pub struct ReceivedCallback {
    pub probe_id: String,
    pub source_ip: std::net::IpAddr,
    pub payload_snippet: String,
    pub received_at: std::time::SystemTime,
}

impl CanaryServer {
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self { bind_addr, inner: Arc::new(Mutex::new(CanaryState::default())) }
    }

    pub fn register_pending(&self, probe_id: &str) {
        self.inner.lock().unwrap().pending.insert(probe_id.to_string(), ());
    }

    pub fn check_received(&self, probe_id: &str) -> bool {
        self.inner.lock().unwrap().received.contains_key(probe_id)
    }

    /// Start listening in the background. Inbound connections are recorded
    /// if their payload contains a known probe ID.
    pub async fn start(&self) {
        let listener = TcpListener::bind(self.bind_addr).await.expect("canary bind failed");
        let inner = self.inner.clone();
        tokio::spawn(async move {
            loop {
                if let Ok((mut stream, peer)) = listener.accept().await {
                    let inner = inner.clone();
                    tokio::spawn(async move {
                        let mut buf = vec![0u8; 4096];
                        let n = stream.read(&mut buf).await.unwrap_or(0);
                        let payload = String::from_utf8_lossy(&buf[..n]).to_string();

                        // Match any pending probe ID that appears in the payload
                        let mut state = inner.lock().unwrap();
                        let matched_id = state.pending.keys()
                            .find(|id| payload.contains(id.as_str()))
                            .cloned();
                        if let Some(id) = matched_id {
                            state.received.insert(id.clone(), ReceivedCallback {
                                probe_id: id,
                                source_ip: peer.ip(),
                                payload_snippet: payload.chars().take(200).collect(),
                                received_at: std::time::SystemTime::now(),
                            });
                        }
                    });
                }
            }
        });
    }
}
```

---

## Phase 5 — Oracle Evaluator

The oracle is the deterministic decision function. It takes a `BehavioralTrace`,
a baseline `BehavioralTrace`, and a `Probe`, and returns a `Verdict`.

### New file: `frensense-runtime/src/oracle.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    /// Definitive confirmation. Confidence in [0.75, 1.0].
    Confirmed { confidence: f64, evidence: OracleEvidence },
    /// No confirmation from this probe. Try next.
    NotConfirmed,
    /// The code path was reached but input was sanitized — this is a true negative.
    SanitizationDetected,
    /// Could not determine (network error, timeout, ambiguous).
    Inconclusive { reason: String },
}

#[derive(Debug, Clone)]
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
            let delta = probe_trace.duration_ms.saturating_sub(baseline_trace.duration_ms);
            if delta >= *threshold_ms {
                // Multiple timing measurements in production — single sample used here.
                // Agents: add statistical jitter guard (run baseline 3x, take median).
                Verdict::Confirmed {
                    confidence: 0.82,
                    evidence: OracleEvidence {
                        oracle_kind: "timing_delta".to_string(),
                        detail: format!("Response delayed {delta}ms above baseline (threshold: {threshold_ms}ms)"),
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
                        detail: format!("Inbound connection received at canary server for probe {probe_id}"),
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

        OracleKind::DifferentialResponse { min_divergence_score } => {
            let div = probe_trace.divergence_from(baseline_trace);
            if div >= *min_divergence_score {
                Verdict::Confirmed {
                    confidence: 0.60 + (div - min_divergence_score) * 0.3,
                    evidence: OracleEvidence {
                        oracle_kind: "differential_response".to_string(),
                        detail: format!("Response divergence {div:.2} exceeds threshold {min_divergence_score:.2}"),
                        raw_value: format!("{div:.4}"),
                    },
                }
            } else {
                // Low divergence with a known-malicious payload means sanitization
                if div < 0.05 {
                    Verdict::SanitizationDetected
                } else {
                    Verdict::NotConfirmed
                }
            }
        }

        OracleKind::StatusCodeChange { expected_original, trigger_different } => {
            let changed = probe_trace.status_code != *expected_original;
            if changed == *trigger_different {
                Verdict::Confirmed {
                    confidence: 0.65,
                    evidence: OracleEvidence {
                        oracle_kind: "status_code_change".to_string(),
                        detail: format!("Status changed from {} to {}", expected_original, probe_trace.status_code),
                        raw_value: probe_trace.status_code.to_string(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }

        OracleKind::BodySizeDelta { min_delta_bytes } => {
            let delta = (probe_trace.response_size_bytes as i64
                - baseline_trace.response_size_bytes as i64).unsigned_abs() as usize;
            if delta >= *min_delta_bytes {
                Verdict::Confirmed {
                    confidence: 0.55,
                    evidence: OracleEvidence {
                        oracle_kind: "body_size_delta".to_string(),
                        detail: format!("Body size changed by {delta} bytes (threshold: {min_delta_bytes})"),
                        raw_value: delta.to_string(),
                    },
                }
            } else {
                Verdict::NotConfirmed
            }
        }
    }
}
```

---

## Phase 6 — Probe Scheduler and Aggregator

Runs probes in priority order. Stops on first definitive confirmation.
Aggregates multiple verdicts into a final `RuntimeAdvisory`.

### New file: `frensense-runtime/src/scheduler.rs`

```rust
pub async fn run_probe_campaign(
    static_advisory: &Advisory,
    route: &RouteBinding,
    template: &ProbeTemplate,
    client: &reqwest::Client,
    canary_server: &CanaryServer,
    config: &RuntimeConfig,
) -> RuntimeAdvisory {

    // Step 1: Capture baseline (benign input, establishes normal behavior)
    let baseline = execute_probe(
        client,
        &ProbeTarget { route, base_url: &config.base_url },
        &Probe {
            id: "baseline".to_string(),
            payload: template.baseline_payload.to_string(),
            oracle: OracleKind::DifferentialResponse { min_divergence_score: 99.0 }, // never fires
            risk: ProbeRisk::Safe,
            description: "Baseline — benign input to establish normal response",
        },
        canary_server,
    ).await;

    // If baseline fails, the endpoint isn't reachable — skip
    if baseline.status_code == 0 {
        return RuntimeAdvisory::inconclusive(static_advisory.clone(), "Endpoint unreachable");
    }

    let mut probe_results: Vec<ProbeResult> = Vec::new();

    // Step 2: Run probes in order, stop on first confirmed
    for probe in &template.probes {
        // Respect risk filter
        if probe.risk > config.max_risk {
            continue;
        }

        let trace = execute_probe(
            client,
            &ProbeTarget { route, base_url: &config.base_url },
            probe,
            canary_server,
        ).await;

        let verdict = evaluate_oracle(&probe.oracle, &trace, &baseline);
        let is_confirmed = matches!(verdict, Verdict::Confirmed { .. });
        let is_sanitized = matches!(verdict, Verdict::SanitizationDetected);

        probe_results.push(ProbeResult {
            probe: probe.clone(),
            trace,
            verdict: verdict.clone(),
        });

        if is_confirmed {
            break; // Stop at first confirmation — don't send more probes
        }
        if is_sanitized {
            break; // Input is sanitized — static FP, stop probing
        }

        // Jitter delay between probes to avoid triggering rate limiting
        tokio::time::sleep(Duration::from_millis(config.inter_probe_delay_ms)).await;
    }

    // Step 3: Aggregate
    build_runtime_advisory(static_advisory.clone(), probe_results)
}

fn build_runtime_advisory(static_advisory: Advisory, results: Vec<ProbeResult>) -> RuntimeAdvisory {
    // Find highest-confidence confirmation
    let confirmed = results.iter().find_map(|r| {
        if let Verdict::Confirmed { confidence, evidence } = &r.verdict {
            Some((*confidence, evidence.clone(), r.probe.clone()))
        } else {
            None
        }
    });

    let sanitization_detected = results.iter().any(|r|
        matches!(r.verdict, Verdict::SanitizationDetected));

    let status = if let Some((confidence, evidence, probe)) = confirmed {
        ConfirmationStatus::Confirmed { confidence, evidence, confirming_probe: probe }
    } else if sanitization_detected {
        ConfirmationStatus::SanitizationDetected
    } else if results.is_empty() {
        ConfirmationStatus::Inconclusive { reason: "No applicable probes for this category".to_string() }
    } else {
        ConfirmationStatus::Unconfirmed
    };

    RuntimeAdvisory {
        static_advisory,
        status,
        probes_attempted: results,
    }
}
```

---

## Phase 7 — Runtime Advisory Output

### New file: `frensense-runtime/src/advisory.rs`

```rust
/// The final output of a runtime verification campaign for one static finding.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RuntimeAdvisory {
    /// The original static finding this probe was generated from
    pub static_advisory: Advisory,
    /// Result of the runtime campaign
    pub status: ConfirmationStatus,
    /// All probes attempted, with their traces and verdicts
    pub probes_attempted: Vec<ProbeResult>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ConfirmationStatus {
    /// At least one probe confirmed the vulnerability with behavioral evidence
    Confirmed {
        confidence: f64,
        evidence: OracleEvidence,
        confirming_probe: Probe,
    },
    /// Probes ran but no oracle fired — vulnerability may be unexploitable in this env
    Unconfirmed,
    /// Injection point was reached but input was sanitized — static finding is a FP
    SanitizationDetected,
    /// Could not probe (endpoint unreachable, no applicable probes, auth required)
    Inconclusive { reason: String },
}

impl RuntimeAdvisory {
    pub fn is_confirmed(&self) -> bool {
        matches!(self.status, ConfirmationStatus::Confirmed { .. })
    }

    /// Combined confidence: static confidence × runtime confirmation confidence
    pub fn combined_confidence(&self) -> f64 {
        if let ConfirmationStatus::Confirmed { confidence, .. } = &self.status {
            (self.static_advisory.confidence * confidence).sqrt() // geometric mean
        } else {
            0.0
        }
    }

    pub fn format_report(&self) -> String {
        let static_adv = &self.static_advisory;
        match &self.status {
            ConfirmationStatus::Confirmed { confidence, evidence, confirming_probe } => format!(
                "[CONFIRMED ✓] {rule} — {file}:{line}\n\
                 Static confidence:  {sc:.0}%\n\
                 Runtime confidence: {rc:.0}%\n\
                 Combined:           {cc:.0}%\n\n\
                 Oracle: {oracle_kind}\n\
                 Evidence: {detail}\n\
                 Probe payload: {payload}\n\n\
                 {observation}\n\
                 Impact: {impact}\n\
                 Fix: {fix}",
                rule        = static_adv.rule_id,
                file        = static_adv.file_path,
                line        = static_adv.line,
                sc          = static_adv.confidence * 100.0,
                rc          = confidence * 100.0,
                cc          = self.combined_confidence() * 100.0,
                oracle_kind = evidence.oracle_kind,
                detail      = evidence.detail,
                payload     = confirming_probe.payload,
                observation = static_adv.observation,
                impact      = static_adv.impact,
                fix         = static_adv.improvement,
            ),
            ConfirmationStatus::SanitizationDetected => format!(
                "[SANITIZED ✗] {rule} — {file}:{line}\n\
                 Static found suspicious code; runtime probes showed input is sanitized.\n\
                 This may be a false positive from the static pass.",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
            ),
            ConfirmationStatus::Unconfirmed => format!(
                "[UNCONFIRMED ?] {rule} — {file}:{line}\n\
                 {n} probes attempted. No oracle fired.\n\
                 The vulnerability may require authentication, specific state, or\n\
                 a different injection vector not covered by the probe library.",
                rule = static_adv.rule_id,
                file = static_adv.file_path,
                line = static_adv.line,
                n    = self.probes_attempted.len(),
            ),
            ConfirmationStatus::Inconclusive { reason } => format!(
                "[INCONCLUSIVE ~] {rule} — {file}:{line}\n\
                 {reason}",
                rule   = static_adv.rule_id,
                file   = static_adv.file_path,
                line   = static_adv.line,
                reason = reason,
            ),
        }
    }
}
```

---

## Crate Structure

```
frensense-runtime/
├── Cargo.toml
└── src/
    ├── main.rs              ← CLI: frensense-runtime --report findings.json --target http://localhost:3000
    ├── lib.rs               ← Public API surface
    ├── config.rs            ← RuntimeConfig (base_url, max_risk, delays, auth)
    ├── route_extractor.rs   ← Phase 1: file path → HTTP endpoint
    ├── probes/
    │   ├── mod.rs           ← Probe, OracleKind, ProbeTemplate, ProbeRisk
    │   ├── cmdi.rs
    │   ├── sqli.rs
    │   ├── ssrf.rs
    │   ├── redirect.rs
    │   ├── path_traversal.rs
    │   └── xss.rs
    ├── tracer.rs            ← BehavioralTrace, execute_probe
    ├── canary.rs            ← CanaryServer (TCP listener)
    ├── oracle.rs            ← evaluate_oracle, Verdict
    ├── scheduler.rs         ← run_probe_campaign, aggregation
    └── advisory.rs          ← RuntimeAdvisory, ConfirmationStatus, format_report
```

---

## CLI Interface

```
frensense-runtime \
  --report findings.json \        ← Output of: frensense scan --format json
  --target http://localhost:3000 \
  --canary-host 1.2.3.4:9999 \   ← Where the CanaryServer will listen
  --max-risk safe \               ← safe | low | medium | destructive
  --auth-header "Authorization: Bearer $TOKEN" \
  --inter-probe-delay 500 \       ← ms between probes (rate limit protection)
  --output runtime-report.json
```

---

## Safety Constraints (Non-Negotiable)

Every probe in the default library must satisfy all of these:

1. **No destructive writes.** No `DROP`, `DELETE`, `rm -rf`, `Format-C`.
   The worst a default probe does is `sleep 5` or print a canary string.

2. **No credential extraction.** Probes that read `/etc/shadow`, `~/.ssh/id_rsa`,
   or environment variables are behind `--max-risk destructive` which
   is off by default and requires `--i-understand-destructive-probes`.

3. **Rate limited by default.** 500ms inter-probe delay, 10 probe max per
   endpoint, 30 endpoint max per session without `--no-limit`.

4. **Scoped to static findings only.** Runtime never probes an endpoint that
   wasn't in the static report. It has zero fuzzing behaviour — it only confirms
   what static already suspects.

5. **Canary strings are UUID-based and expire.** Each probe run generates fresh
   UUIDs. Old canary strings cannot be replayed against future sessions.

---

## Corpus Integration — How Probes Stay in Sync with Patterns

The same `motifs.rs` introduced in the Intelligence guide drives both the
static engine and the runtime probe selector:

```rust
// In scheduler.rs, selecting probe template:
let motif = motif_for_advisory(advisory);  // reads rule_id → motif name
let template = template_for_motif(motif, &canary_server.bind_addr.to_string());

// motif_for_advisory:
fn motif_for_advisory(adv: &Advisory) -> &'static str {
    match category_from_rule_id(&adv.rule_id) {
        "cmdi"           => "CommandExecutionSink",
        "sqli"           => "SqlSink",
        "ssrf"           => "HttpOutboundSink",
        "redirect"       => "HttpResponseSink",
        "path_traversal" => "FileReadSink",
        "xss"            => "EvalSink",
        _                => "Unknown",
    }
}
```

When you add a new motif to the static engine's `MOTIFS` registry, you add
the corresponding probe template to `frensense-runtime/src/probes/`. The
two lists stay parallel by design.

---

## Categories Covered vs. Not Covered

### Runtime-verifiable (HTTP-based injection)

| Rule prefix | Category | Oracle type |
|---|---|---|
| `*_cmdi_*` | Command injection | Canary echo, timing |
| `*_sqli_*` | SQL injection | Error pattern, timing, differential |
| `*_ssrf_*` | SSRF | Callback server |
| `*_redirect_*` | Open redirect | Location header |
| `*_path_traversal_*` | Path traversal | /etc/passwd canary in body |
| `*_xss_*` | XSS (reflected) | Canary in body |

### Requires additional context (implement later)

| Rule prefix | Category | Why harder |
|---|---|---|
| `*_idor_*` | IDOR | Needs two user sessions + different resource IDs |
| `*_auth_*` | Auth bypass | Needs knowledge of protected routes + credentials |
| `tsx_useeffect_*` | React hook bugs | Browser runtime, not HTTP |
| `rust_async_*` | Async correctness | No HTTP surface — process-level |
| `*_crypto_*` | Weak crypto | Output analysis, not injection |

For IDOR, the runtime can still attempt probing if the static finding provides
two `enclosing_symbol` entries and the route pattern has a `:id` segment. Swap
the ID to `id + 1` and check if the response returns data. This is lower
confidence without a second auth token.

---

## Build Order

1. `advisory.rs` — data structures only, no dependencies
2. `canary.rs` — TCP listener, depends only on `tokio`
3. `probes/mod.rs` + individual probe files — pure data
4. `tracer.rs` — depends on `probes`, `canary`
5. `oracle.rs` — depends on `tracer`, `probes`
6. `route_extractor.rs` — depends on `tree-sitter` (already in workspace)
7. `scheduler.rs` — depends on all of the above
8. `main.rs` — CLI wiring, depends on `frensense` (existing crate) for `Advisory`

The `frensense-runtime` crate depends on the main `frensense` crate only for
the `Advisory` struct and the `motifs` module. It does not re-run static
analysis — it reads the JSON report that Frensense Static already produced.
