use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use frensense_runtime::canary::CanaryServer;
use frensense_runtime::oracle::{evaluate_oracle, Verdict};
use frensense_runtime::probes::{OracleKind, Probe, ProbeRisk};
use frensense_runtime::route_extractor::{
    Framework, HttpMethod, InjectionPoint, ParameterLocation, RouteBinding,
};
use frensense_runtime::tracer::{execute_probe, BehavioralTrace, ProbeTarget};

fn free_port() -> u16 {
    let listener = StdTcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

/// A minimal HTTP server that echoes back the request body / query params / headers
/// based on the URL path. Supports:
///   POST /echo-body        — returns request body as response
///   GET /echo-query        — returns query params as JSON
///   GET /echo-header       — returns a specific header value
///   POST /canary-in-body   — reflects a canary string in response
///   POST /sleep            — sleeps for `ms` ms
///   GET /status/:code      — returns given status code
async fn run_mock_server(port: u16) {
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = TcpListener::bind(addr).await.unwrap();

    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                let n = stream.read(&mut buf).await.unwrap_or(0);
                if n == 0 {
                    return;
                }
                let request = String::from_utf8_lossy(&buf[..n]).to_string();

                let response = if request.contains("GET /echo-query") {
                    // Parse query params from the request line
                    let params = if let Some(qs) = request.lines().next()
                        .and_then(|l| l.split('?').nth(1))
                        .and_then(|s| s.split(' ').next())
                    {
                        qs.to_string()
                    } else {
                        String::new()
                    };
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{{\"query\":\"{}\"}}",
                        params.len() + 11,
                        params
                    )
                } else if request.contains("POST /canary-in-body") {
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    )
                } else if request.contains("POST /sleep") {
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("{}");
                    // Parse sleep duration from JSON body
                    let ms: u64 = if body.contains("\"ms\":") {
                        body.split("\"ms\":")
                            .nth(1)
                            .map(|s| s.trim_start().trim_end_matches('}').trim_matches('"'))
                            .and_then(|s| s.split(|c: char| !c.is_ascii_digit()).next())
                            .and_then(|s| if s.is_empty() { None } else { s.parse().ok() })
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 6\r\n\r\nSlept {}ms",
                        ms
                    )
                } else if request.contains("POST /echo-body") {
                    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    )
                } else if request.contains("GET /status/") {
                    let code: u16 = request
                        .lines()
                        .next()
                        .and_then(|l| l.split(" /status/").nth(1))
                        .and_then(|s| s.split(' ').next())
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(200);
                    let reason = if code == 200 { "OK" } else { "Error" };
                    format!(
                        "HTTP/1.1 {} {}\r\nContent-Length: 0\r\n\r\n",
                        code, reason
                    )
                } else if request.contains("GET /redirect") {
                    format!(
                        "HTTP/1.1 302 Found\r\nLocation: http://canary-target/probe\r\nContent-Length: 0\r\n\r\n"
                    )
                } else {
                    format!(
                        "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n"
                    )
                };

                let _ = stream.write_all(response.as_bytes()).await;
            });
        }
    });
}

fn make_route(path: &str, method: HttpMethod) -> RouteBinding {
    RouteBinding {
        method,
        path_pattern: path.to_string(),
        handler_file: "test.ts".to_string(),
        handler_function: "handler".to_string(),
        injection_points: vec![InjectionPoint {
            location: ParameterLocation::Body,
            name: "input".to_string(),
            taint_origin: Some("user_input".to_string()),
        }],
        framework: Framework::Express,
    }
}

fn make_canary_probe(canary: &str) -> Probe {
    Probe {
        id: "test-probe".to_string(),
        payload: canary.to_string(),
        oracle: OracleKind::CanaryInBody {
            canary: canary.to_string(),
        },
        risk: ProbeRisk::Safe,
        description: "Test probe".to_string(),
    }
}

#[tokio::test]
async fn test_canary_in_body_oracle() {
    let port = free_port();
    run_mock_server(port).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], 0).into());
    canary_server.start().await;

    let canary = "FRENSENSE_TEST_CANARY";
    let body_point = InjectionPoint {
        location: ParameterLocation::Body,
        name: "input".to_string(),
        taint_origin: None,
    };

    let trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &make_route("/canary-in-body", HttpMethod::Post),
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &body_point,
            auth: None,
        },
        &make_canary_probe(canary),
        &canary_server,
    )
    .await;

    assert_eq!(trace.status_code, 200);
    let body_str = String::from_utf8_lossy(&trace.response_body);
    assert!(body_str.contains(canary), "Response body should contain canary");
}

#[tokio::test]
async fn test_timing_delta_oracle() {
    let port = free_port();
    run_mock_server(port).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], 0).into());
    canary_server.start().await;

    let sleep_route = RouteBinding {
        method: HttpMethod::Post,
        path_pattern: "/sleep".to_string(),
        handler_file: "test.ts".to_string(),
        handler_function: "handler".to_string(),
        injection_points: vec![InjectionPoint {
            location: ParameterLocation::Body,
            name: "ms".to_string(),
            taint_origin: None,
        }],
        framework: Framework::Express,
    };

    let baseline_trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &sleep_route,
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &InjectionPoint {
                location: ParameterLocation::Body,
                name: "ms".to_string(),
                taint_origin: None,
            },
            auth: None,
        },
        &Probe {
            id: "baseline".to_string(),
            payload: "0".to_string(),
            oracle: OracleKind::TimingDelta { threshold_ms: 9999 },
            risk: ProbeRisk::Safe,
            description: "baseline".to_string(),
        },
        &canary_server,
    )
    .await;

    let probe_trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &sleep_route,
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &InjectionPoint {
                location: ParameterLocation::Body,
                name: "ms".to_string(),
                taint_origin: None,
            },
            auth: None,
        },
        &Probe {
            id: "timing-probe".to_string(),
            payload: "2000".to_string(),
            oracle: OracleKind::TimingDelta { threshold_ms: 1500 },
            risk: ProbeRisk::Safe,
            description: "timing test".to_string(),
        },
        &canary_server,
    )
    .await;

    let verdict = evaluate_oracle(
        &OracleKind::TimingDelta {
            threshold_ms: 1500,
        },
        &probe_trace,
        &baseline_trace,
    );

    assert!(
        matches!(verdict, Verdict::Confirmed { .. }),
        "Timing oracle should fire when probe is slower than baseline. baseline={}ms probe={}ms",
        baseline_trace.duration_ms,
        probe_trace.duration_ms,
    );
}

#[tokio::test]
async fn test_error_pattern_oracle() {
    let port = free_port();
    run_mock_server(port).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], 0).into());
    canary_server.start().await;

    let body_point = InjectionPoint {
        location: ParameterLocation::Body,
        name: "input".to_string(),
        taint_origin: None,
    };

    let trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &make_route("/canary-in-body", HttpMethod::Post),
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &body_point,
            auth: None,
        },
        &Probe {
            id: "error-probe".to_string(),
            payload: "status: 500 error occurred".to_string(),
            oracle: OracleKind::ErrorPattern {
                patterns: vec!["error".to_string(), "syntax".to_string()],
            },
            risk: ProbeRisk::Safe,
            description: "error probe".to_string(),
        },
        &canary_server,
    )
    .await;

    let baseline = BehavioralTrace {
        probe_id: "baseline".to_string(),
        payload: String::new(),
        status_code: 200,
        response_body: Vec::new(),
        response_body_hash: 0,
        response_size_bytes: 0,
        response_headers: HashMap::new(),
        duration_ms: 0,
        canary_received: false,
        redirect_location: None,
        transport_error: None,
        sent_at: std::time::SystemTime::now(),
    };

    let verdict = evaluate_oracle(
        &OracleKind::ErrorPattern {
            patterns: vec!["error".to_string(), "syntax".to_string()],
        },
        &trace,
        &baseline,
    );

    assert!(
        matches!(verdict, Verdict::Confirmed { .. }),
        "Error pattern oracle should fire when response contains error keyword"
    );
}

#[tokio::test]
async fn test_injection_point_building() {
    let port = free_port();
    run_mock_server(port).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], 0).into());
    canary_server.start().await;

    // Test query parameter injection
    let query_point = InjectionPoint {
        location: ParameterLocation::Query,
        name: "q".to_string(),
        taint_origin: None,
    };

    let trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &make_route("/echo-query", HttpMethod::Get),
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &query_point,
            auth: None,
        },
        &Probe {
            id: "query-probe".to_string(),
            payload: "test_value".to_string(),
            oracle: OracleKind::CanaryInBody {
                canary: "q=test_value".to_string(),
            },
            risk: ProbeRisk::Safe,
            description: "query probe".to_string(),
        },
        &canary_server,
    )
    .await;

    let body_str = String::from_utf8_lossy(&trace.response_body);
    assert!(
        body_str.contains("test_value"),
        "Query param injection should pass the value. Response: {body_str}"
    );
}

#[tokio::test]
async fn test_status_code_change_oracle() {
    let port = free_port();
    run_mock_server(port).await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let client = reqwest::Client::new();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], 0).into());
    canary_server.start().await;

    let status_route = RouteBinding {
        method: HttpMethod::Get,
        path_pattern: "/status/500".to_string(),
        handler_file: "test.ts".to_string(),
        handler_function: "handler".to_string(),
        injection_points: Vec::new(),
        framework: Framework::Express,
    };

    let trace = execute_probe(
        &client,
        &ProbeTarget {
            route: &status_route,
            base_url: &format!("http://127.0.0.1:{port}"),
            injection_point: &InjectionPoint {
                location: ParameterLocation::Body,
                name: "input".to_string(),
                taint_origin: None,
            },
            auth: None,
        },
        &Probe {
            id: "status-probe".to_string(),
            payload: String::new(),
            oracle: OracleKind::StatusCodeChange {
                expected_original: 200,
                trigger_different: true,
            },
            risk: ProbeRisk::Safe,
            description: "status probe".to_string(),
        },
        &canary_server,
    )
    .await;

    let baseline = BehavioralTrace {
        probe_id: "baseline".to_string(),
        payload: String::new(),
        status_code: 200,
        response_body: Vec::new(),
        response_body_hash: 0,
        response_size_bytes: 0,
        response_headers: HashMap::new(),
        duration_ms: 0,
        canary_received: false,
        redirect_location: None,
        transport_error: None,
        sent_at: std::time::SystemTime::now(),
    };

    let verdict = evaluate_oracle(
        &OracleKind::StatusCodeChange {
            expected_original: 200,
            trigger_different: true,
        },
        &trace,
        &baseline,
    );

    assert!(
        matches!(verdict, Verdict::Confirmed { .. }),
        "Status code oracle should fire when status changed from 200 to 500"
    );
}

#[tokio::test]
async fn test_canary_server_tcp_callback() {
    let canary_port = free_port();
    let canary_server = CanaryServer::new(([127, 0, 0, 1], canary_port).into());
    canary_server.start().await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let probe_id = "test-callback-probe-id-12345";
    canary_server.register_pending(probe_id);

    // Simulate an inbound TCP connection with the probe ID in the payload
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let addr: SocketAddr = ([127, 0, 0, 1], canary_port).into();
    let mut stream = tokio::net::TcpStream::connect(addr)
        .await
        .unwrap();
    let payload = format!("GET /frensense-probe?pid={probe_id} HTTP/1.1\r\nHost: test\r\n\r\n");
    let _ = stream.write_all(payload.as_bytes()).await;

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert!(
        canary_server.check_received(probe_id),
        "Canary server should have received the callback for {probe_id}"
    );
}
