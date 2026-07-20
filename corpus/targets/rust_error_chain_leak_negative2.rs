// SAFE: Uses a custom error type that never exposes internal details in its Display
use axum::{response::IntoResponse, Json};
use std::fmt;

struct AppError(anyhow::Error);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "internal server error")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::json!({"error": self.to_string()});
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}
