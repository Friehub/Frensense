// SAFE: Validates the deserialized struct using the `validator` crate before processing
use axum::{Router, routing::post, Json, http::StatusCode};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct TransferRequest {
    #[validate(length(min = 10, max = 20))]
    pub from_account: String,
    #[validate(length(min = 10, max = 20))]
    pub to_account: String,
    #[validate(range(min = 0.01))]
    pub amount: f64,
    #[validate(regex = "^(USD|EUR|GBP)$")]
    pub currency: String,
}

async fn transfer(Json(payload): Json<TransferRequest>) -> Result<&'static str, StatusCode> {
    if payload.validate().is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok("Transfer processed")
}

async fn serve() {
    let app = Router::new().route("/transfer", post(transfer));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
