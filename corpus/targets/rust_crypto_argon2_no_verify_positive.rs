// [frensense]
// observation: An Argon2 password hash is created but never verified against the user-provided password during login, meaning any password might be accepted.
// impact: An attacker can log in as any user without knowing their password, since the hash is never checked.
// improvement: Always verify the password against the stored hash using argon2::verify.

use argon2::{self, Config};

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = b"randomsalt12345678";
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config)
}

fn register_user(username: &str, password: &str) -> Result<(), std::io::Error> {
    let hash = hash_password(password).unwrap();
    store_user(username, &hash)
}

fn login_user(username: &str, password: &str) -> bool {
    let stored = get_user_hash(username);
    let hash = hash_password(password).unwrap();
    hash == stored
}
