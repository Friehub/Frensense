// SAFE: Uses OpenOptions with mode set explicitly before creation, ensuring the file is created with the correct permissions atomically.

use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

fn write_secret(path: &str, data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn create_token_file(path: &str, token: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(token.as_bytes())?;
    Ok(())
}
