use crate::Advisory;

pub fn find(_snap: &crate::engine::project::FileSnapshot) -> Vec<Advisory> {
    // Temporal detection is now corpus-driven (rust_temporal_lock_unlock, rust_temporal_lock_sleep, etc.)
    // No hardcoded rules needed.
    Vec::new()
}
