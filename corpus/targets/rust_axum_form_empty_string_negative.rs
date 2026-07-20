use axum::{Router, routing::post, extract::Form, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    #[serde(default)]
    name: Option<String>,
}

async fn handler(Form(input): Form<Input>) -> impl IntoResponse {
    // SAFE: Explicitly check for None or empty string.
    match input.name.filter(|n| !n.is_empty()) {
        Some(name) => format!("hello {name}."),
        None => "name is required".into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
