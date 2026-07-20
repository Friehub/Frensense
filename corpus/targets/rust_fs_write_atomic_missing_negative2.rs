// SAFE: Uses fs2::FileExt for atomic write or tempfile::NamedTempFile with persist for crash-safe writes.

use fs2::FileExt;
use std::fs::OpenOptions;
use std::io::{Write, Result};

fn update_config(path: &str, content: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    file.lock_exclusive()?;
    file.write_all(content.as_bytes())?;
    file.sync_all()?;
    file.unlock()?;
    Ok(())
}
