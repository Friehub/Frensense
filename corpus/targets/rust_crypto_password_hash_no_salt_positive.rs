// [frensense]
// observation: A password hash is computed using `bcrypt::hash` or similar without an explicit salt, relying on the default (or empty salt). Without a unique per-password salt, identical passwords produce identical hashes.
// impact: Rainbow table attacks become feasible, and attackers can identify users with the same password across the system.
// improvement: Always use a function that generates a unique random salt per password (e.g., `bcrypt::hash` with default cost includes a salt).

use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
