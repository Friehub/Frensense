// SAFE: bcrypt::hash automatically generates a random salt per call.
use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
