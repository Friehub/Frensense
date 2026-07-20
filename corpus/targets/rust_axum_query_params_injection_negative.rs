use axum::{Router, extract::Query, routing::get, response::IntoResponse};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct SearchParams {
    #[validate(length(min = 1, max = 100))]
    query: String,
    #[validate(range(min = 1, max = 1000))]
    page: u32,
}

async fn search(params: Query<SearchParams>) -> impl IntoResponse {
    // SAFE: validate() checks length and range constraints before use.
    if params.validate().is_err() {
        return axum::response::Response::new(axum::body::Body::from("invalid params"));
    }
    format!("searching for {} on page {}", params.query, params.page)
}

pub fn app() -> Router {
    Router::new().route("/search", get(search))
}
