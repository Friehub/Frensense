use axum::{response::{IntoResponse, Response}, http::StatusCode};
use std::fmt;

enum AppError {
    NotFound,
    Database(sqlx::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "not found"),
            AppError::Database(_) => write!(f, "internal error"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        // SAFE: `Display` impl only exposes safe messages, never raw error internals.
        (status, self.to_string()).into_response()
    }
}

async fn handler() -> Result<String, AppError> {
    Err(AppError::NotFound)
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
