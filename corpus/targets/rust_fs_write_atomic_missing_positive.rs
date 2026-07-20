// [frensense]
// observation: A file is written directly to its final path without using an atomic write pattern (write to temp then rename), risking partial or corrupted data on crash.
// impact: If the process crashes mid-write or power is lost, the file contains truncated or partial data, causing data corruption that may go undetected.
// improvement: Write to a temporary file in the same directory, then atomically rename (fs::rename) to the target path.

use std::fs;
use std::io;

fn update_config(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

fn save_user_data(path: &str, data: &[u8]) -> io::Result<()> {
    fs::write(path, data)?;
    Ok(())
}
