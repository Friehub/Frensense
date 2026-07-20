// SAFE: Uses write-to-temp-then-rename pattern to ensure atomic file writes; on crash the original file is preserved.

use std::fs;
use std::io;

fn update_config(path: &str, content: &str) -> io::Result<()> {
    let tmp_path = format!("{}.tmp", path);
    fs::write(&tmp_path, content)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn save_user_data(path: &str, data: &[u8]) -> io::Result<()> {
    let tmp_path = format!("{}.tmp", path);
    fs::write(&tmp_path, data)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}
