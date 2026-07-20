// [frensense]
// observation: An Axum handler uses the `Query` extractor to parse URL query parameters into a struct without validating the deserialized fields. The struct fields are used directly in sensitive operations such as database queries or file access.
// impact: Query parameter injection — an attacker can supply unexpected values (e.g., SQL fragments, path traversal sequences, or special characters) that bypass application logic, leading to SQL injection, path traversal, or logic bypass.
// improvement: Validate all extracted query parameters before use. Use a validation library (e.g., `validator`, `garde`) or manual checks on each field.

use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    query: String,
    page: u32,
}

async fn search(params: Query<SearchParams>) -> String {
    format!("searching for {} on page {}", params.query, params.page)
}

pub fn app() -> Router {
    Router::new().route("/search", get(search))
}
