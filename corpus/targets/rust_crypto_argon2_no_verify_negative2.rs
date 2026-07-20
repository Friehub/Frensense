// SAFE: Uses argon2 with proper hash verification using the Config's verified flag
use argon2::{self, Config, ThreadMode, Variant};

fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = b"randomsalt12345678";
    let config = Config {
        variant: Variant::Argon2id,
        hash_length: 32,
        lanes: 4,
        mem_cost: 65536,
        time_cost: 3,
        thread_mode: ThreadMode::Parallel,
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), salt, &config)
}

fn verify_password(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}
