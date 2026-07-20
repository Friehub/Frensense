// [frensense]
// observation: An `IntoResponse` implementation for a custom error type formats the internal error message (including file paths, SQL queries, or stack traces) directly into the HTTP response body without sanitization.
// impact: Internal details exposed to the client aid attackers in crafting further exploits. Database schema, file paths, dependency versions, and library internals can be inferred from error messages — reducing the effort required for targeted attacks.
// improvement: Map internal errors to generic user-facing messages while logging the full details server-side.

use axum::{response::{IntoResponse, Response}, http::StatusCode};

struct AppError {
    inner: sqlx::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.inner.to_string()).into_response()
    }
}

async fn query_db() -> Result<(), AppError> {
    Err(AppError {
        inner: sqlx::Error::Database(Box::new(
            sqlx::error::Error::Protocol("relation \"users\" does not exist".into())
        )),
    })
}

async fn handler() -> Result<String, AppError> {
    query_db().await?;
    Ok("ok".into())
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
