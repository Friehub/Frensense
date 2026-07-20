// [frensense]
// observation: jsonwebtoken::decode is called instead of jsonwebtoken::verify, which decodes the token without validating the signature.
// impact: An attacker can forge arbitrary tokens with any claims, bypassing authentication entirely.
// improvement: Use jsonwebtoken::decode with a proper Validation struct that verifies the signature, issuer, and audience.

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn parse_token(token: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(b"secret");
    let data = decode::<serde_json::Value>(token, &key, &Validation::default())?;
    Ok(data.claims)
}

fn decode_unsafe(token_str: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(token_str, &DecodingKey::from_secret(b"key"), &Validation::new(Algorithm::HS256))?;
    Ok(data.claims)
}
