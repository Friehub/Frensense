// SAFE: Uses decode with Validation that requires signature verification and disables insecure algorithms
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 0;
    validation.required_spec_claims = ["sub", "exp", "iat"].iter().cloned().collect();
    let data = decode::<Claims>(token, &DecodingKey::from_secret(b"strong-secret-123"), &validation)?;
    Ok(data.claims)
}
