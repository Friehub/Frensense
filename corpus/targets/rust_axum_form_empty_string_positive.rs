// [frensense]
// observation: Axum's `Form` extractor deserializes empty strings as present values (e.g., `""` for a `String` field). The application treats the presence of the field as validation that the user provided meaningful input.
// impact: An empty string passes validation that only checks `Option::is_some`, allowing blank values into business logic. This can cause downstream issues: database write of empty fields, incorrect display, broken downstream integrations.
// improvement: Validate string length after extraction, or use a custom extractor that rejects empty strings.

use axum::{Router, routing::post, extract::Form};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    name: String,
}

async fn handler(Form(input): Form<Input>) -> String {
    format!("hello {}.", input.name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
