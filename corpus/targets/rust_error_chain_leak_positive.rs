// [frensense]
// observation: The full error chain from anyhow::Error or similar is serialized into an HTTP response, leaking internal implementation details.
// impact: Internal file paths, SQL queries, network addresses, and stack traces are exposed to the client, aiding further attacks.
// improvement: Map errors to user-friendly messages before responding, and log the full error chain server-side only.

use axum::{response::{IntoResponse, Response}, Json};
use anyhow::Context;

async fn handler() -> Result<Json<serde_json::Value>, AppError> {
    let data = fetch_data().await?;
    Ok(Json(data))
}

fn handle_error(err: anyhow::Error) -> Response {
    let body = serde_json::json!({"error": format!("{:?}", err)});
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
}
