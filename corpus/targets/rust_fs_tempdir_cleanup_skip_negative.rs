// SAFE: TempDir is explicitly dropped before early returns using scopeguard::defer! to guarantee cleanup.

use std::fs;
use std::io;
use scopeguard::defer;
use tempfile::TempDir;

fn process_archive(path: &str) -> io::Result<String> {
    let tmp = TempDir::new()?;
    defer! { drop(tmp) };
    let file_path = tmp.path().join("extracted");
    let data = fs::read_to_string(path)?;
    fs::write(&file_path, data)?;
    if file_path.exists() {
        return Ok("done".into());
    }
    Ok("incomplete".into())
}

fn convert_file(input: &str) -> io::Result<()> {
    let tmp = TempDir::new()?;
    defer! { drop(tmp) };
    let output = tmp.path().join("converted");
    fs::copy(input, &output)?;
    fs::remove_file(input)?;
    Ok(())
}
