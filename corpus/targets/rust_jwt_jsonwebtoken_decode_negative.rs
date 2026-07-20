// SAFE: Uses validate_headers and proper Validation with issuer and audience checks
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn verify_token(token: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&["myapp"]);
    validation.set_audience(&["myapi"]);
    validation.required_spec_claims = std::collections::HashSet::from(["sub", "exp", "iss"]);
    let key = DecodingKey::from_secret(b"real-secret-key");
    let data = decode::<serde_json::Value>(token, &key, &validation)?;
    Ok(data.claims)
}
