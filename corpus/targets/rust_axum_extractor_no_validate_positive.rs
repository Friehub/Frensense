// [frensense]
// observation: Axum `Json<T>` or `Query<T>` extractor is used without any validation of the deserialized struct, accepting arbitrary or malicious input.
// impact: An attacker can inject values that pass type-level deserialization but violate business logic, such as negative prices, SQL injection payloads, or privilege escalation through role fields.
// improvement: Add a validation step (e.g. using `validator` crate or a manual `Validate` trait) after extraction.

use axum::{Router, routing::post, Json};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub from_account: String,
    pub to_account: String,
    pub amount: f64,
    pub currency: String,
}

async fn transfer(Json(payload): Json<TransferRequest>) -> &'static str {
    "Transfer processed"
}

async fn serve() {
    let app = Router::new().route("/transfer", post(transfer));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
