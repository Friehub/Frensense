// [frensense]
// observation: tempfile::TempDir is created but not explicitly dropped or cleaned up when the function returns early via ? or early return, leaving temporary files on disk.
// impact: Temporary files accumulate on disk, consuming disk space and potentially leaking sensitive data processed in the temp directory.
// improvement: Ensure TempDir is bound to a variable that lives long enough, or use scopeguard::defer! to clean up on early return.

use std::fs;
use std::io;
use tempfile::TempDir;

fn process_archive(path: &str) -> io::Result<String> {
    let tmp = TempDir::new()?;
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
    let output = tmp.path().join("converted");
    fs::copy(input, &output)?;
    fs::remove_file(input)?;
    Ok(())
}
