use axum::{Router, extract::Query, routing::get, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    query: String,
    page: u32,
}

fn sanitize_query(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

async fn search(params: Query<SearchParams>) -> impl IntoResponse {
    // SAFE: Manual sanitization strips special characters before query is used.
    let safe_query = sanitize_query(&params.query);
    if params.page == 0 || params.page > 1000 {
        return axum::response::Response::new(axum::body::Body::from("invalid page"));
    }
    format!("searching for {} on page {}", safe_query, params.page)
}

pub fn app() -> Router {
    Router::new().route("/search", get(search))
}
