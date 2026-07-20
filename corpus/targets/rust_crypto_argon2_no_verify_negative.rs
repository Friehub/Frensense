// SAFE: Uses argon2::verify_encoded to properly verify the password against stored hash
use argon2;

fn login_user(username: &str, password: &str) -> bool {
    let stored = get_user_hash(username);
    argon2::verify_encoded(&stored, password.as_bytes()).unwrap_or(false)
}
