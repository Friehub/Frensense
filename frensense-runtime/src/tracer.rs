use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::adapters::AuthConvention;
use crate::canary::CanaryServer;
use crate::probes::{OracleKind, Probe};
use crate::route_extractor::{InjectionPoint, ParameterLocation, RouteBinding};
use crate::route_extractor::{HttpMethod};
use crate::session::Session;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub redirect_location: Option<String>,
    pub transport_error: Option<String>,
    pub sent_at: std::time::SystemTime,
}

pub struct ProbeTarget<'a> {
    pub route: &'a RouteBinding,
    pub base_url: &'a str,
    pub injection_point: &'a InjectionPoint,
    pub auth: Option<(&'a AuthConvention, &'a str)>,
    pub session: Option<&'a Session>,
}

impl BehavioralTrace {
    pub fn divergence_from(&self, baseline: &BehavioralTrace) -> f64 {
        let mut score = 0.0f64;
        let mut weight_total = 0.0f64;

        let status_weight = 0.35;
        if self.status_code != baseline.status_code {
            score += status_weight;
        }
        weight_total += status_weight;

        let size_weight = 0.25;
        let size_delta =
            (self.response_size_bytes as i64 - baseline.response_size_bytes as i64).unsigned_abs();
        let size_ratio = size_delta as f64 / (baseline.response_size_bytes.max(1) as f64);
        score += size_weight * size_ratio.min(1.0);
        weight_total += size_weight;

        let body_weight = 0.25;
        if self.response_body_hash != baseline.response_body_hash {
            score += body_weight;
        }
        weight_total += body_weight;

        let timing_weight = 0.15;
        let timing_delta = self.duration_ms.saturating_sub(baseline.duration_ms);
        if timing_delta > 1000 {
            score += timing_weight * (timing_delta as f64 / 5000.0).min(1.0);
        }
        weight_total += timing_weight;

        if weight_total > 0.0 {
            score / weight_total
        } else {
            0.0
        }
    }
}

fn hash_bytes(data: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut h);
    h.finish()
}

pub async fn execute_probe(
    client: &reqwest::Client,
    target: &ProbeTarget<'_>,
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
            let headers: HashMap<String, String> = resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();
            let redirect_location = headers.get("location").cloned();
            let body = resp.bytes().await.unwrap_or_default().to_vec();
            let body_hash = hash_bytes(&body);

            let canary_received = if matches!(probe.oracle, OracleKind::CanaryCallback { .. }) {
                tokio::time::sleep(Duration::from_millis(500)).await;
                canary_server.check_received(&probe_id)
            } else {
                canary_server.check_received(&probe_id)
            };

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

fn build_request(
    client: &reqwest::Client,
    target: &ProbeTarget<'_>,
    payload: &str,
) -> reqwest::RequestBuilder {
    let base_url = target.base_url.trim_end_matches('/');
    let point = target.injection_point;
    let method: reqwest::Method = match target.route.method {
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
        HttpMethod::Delete => reqwest::Method::DELETE,
        HttpMethod::Patch => reqwest::Method::PATCH,
        HttpMethod::All => reqwest::Method::POST,
    };

    let request = match &point.location {
        ParameterLocation::Body => {
            let body = serde_json::json!({ &point.name: payload });
            client.request(method, &format!("{}{}", base_url, target.route.path_pattern)).json(&body)
        }
        ParameterLocation::Query => {
            client.request(method, &format!("{}{}", base_url, target.route.path_pattern))
                .query(&[(&point.name, payload)])
        }
        ParameterLocation::PathParam => {
                    let url = format!("{}{}", base_url, target.route.path_pattern)
                .replace(&format!(":{}", point.name), &urlencoding::encode(payload).into_owned());
            client.request(method, &url)
        }
        ParameterLocation::Header => {
            client.request(method, &format!("{}{}", base_url, target.route.path_pattern))
                .header(&point.name, payload)
        }
        ParameterLocation::Cookie => {
            client.request(method, &format!("{}{}", base_url, target.route.path_pattern))
                .header("Cookie", format!("{}={}", point.name, payload))
        }
        ParameterLocation::FormData => {
            let params = [(&point.name, payload)];
            client.request(method, &format!("{}{}", base_url, target.route.path_pattern))
                .form(&params)
        }
    };

    let request = if let Some((auth, token)) = target.auth {
        auth.apply_auth(request, token)
    } else {
        request
    };

    if let Some(session) = target.session {
        crate::session::apply_session_to_request(session, request)
    } else {
        request
    }
}


