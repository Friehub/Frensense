// SAFE: Maps errors to user-friendly messages, logs the full chain server-side
use axum::{response::{IntoResponse, Response}, Json};
use tracing::error;

fn handle_error(err: anyhow::Error) -> Response {
    error!("internal error: {:?}", err);
    let body = serde_json::json!({"error": "an internal error occurred"});
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
}
