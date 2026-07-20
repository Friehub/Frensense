// [frensense]
// observation: `std::fs::write` is called with a path derived from user input without validating the path, allowing arbitrary file writes.
// impact: An attacker can overwrite critical system files, configuration files, or source code by controlling the path, potentially leading to code execution or denial of service.
// improvement: Validate the path against an allowlist of permitted directories or sanitize it to prevent directory traversal.

use std::fs;
use std::path::PathBuf;

fn save_upload(filename: String, data: Vec<u8>) -> std::io::Result<()> {
    let path = PathBuf::from("/var/www/uploads").join(&filename);
    fs::write(&path, &data)?;
    Ok(())
}

fn save_config(user_path: String, content: String) -> std::io::Result<()> {
    fs::write(&user_path, &content)?;
    Ok(())
}
