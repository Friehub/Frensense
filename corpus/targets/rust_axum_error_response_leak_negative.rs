use axum::{response::{IntoResponse, Response}, http::StatusCode};

struct AppError {
    inner: sqlx::Error,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("internal error: {}", self.inner);
        // SAFE: Only generic message exposed to client; details logged server-side.
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
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
