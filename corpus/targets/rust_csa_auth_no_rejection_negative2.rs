// SAFE: Returns Option<AuthResult> directly with None on every failure path
struct AuthResult {
    id: u64,
    name: String,
    role: String,
}

fn authenticate_user(token: &str, secret: &str, now: u64) -> Option<AuthResult> {
    if token.is_empty() {
        return None;
    }

    let decoded = decode_jwt(token, secret).ok()?;

    if decoded.exp < now {
        return None;
    }

    if decoded.iss != "auth.example.com" {
        return None;
    }

    Some(AuthResult {
        id: decoded.sub.parse().ok()?,
        name: decoded.name,
        role: "user".to_string(),
    })
}
