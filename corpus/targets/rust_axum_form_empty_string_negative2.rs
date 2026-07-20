use axum::{Router, routing::post, extract::{Form, FromRequest}, http::StatusCode, response::{IntoResponse, Response}};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    name: String,
}

async fn handler(Form(input): Form<Input>) -> Result<String, StatusCode> {
    // SAFE: Validate at the handler boundary, rejecting empty strings.
    if input.name.is_empty() {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    Ok(format!("hello {}.", input.name))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
