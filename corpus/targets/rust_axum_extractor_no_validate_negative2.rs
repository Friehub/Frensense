// SAFE: Manual validation function checks each field before processing
use axum::{Router, routing::post, Json, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub from_account: String,
    pub to_account: String,
    pub amount: f64,
    pub currency: String,
}

fn validate_transfer(req: &TransferRequest) -> Result<(), &'static str> {
    if req.from_account.len() < 10 || req.from_account.len() > 20 {
        return Err("invalid from_account");
    }
    if req.to_account.len() < 10 || req.to_account.len() > 20 {
        return Err("invalid to_account");
    }
    if req.amount < 0.01 {
        return Err("amount must be positive");
    }
    match req.currency.as_str() {
        "USD" | "EUR" | "GBP" => Ok(()),
        _ => Err("unsupported currency"),
    }
}

async fn transfer(Json(payload): Json<TransferRequest>) -> Result<&'static str, (StatusCode, String)> {
    validate_transfer(&payload).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok("Transfer processed")
}

async fn serve() {
    let app = Router::new().route("/transfer", post(transfer));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
