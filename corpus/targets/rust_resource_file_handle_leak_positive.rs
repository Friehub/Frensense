// [frensense]
// observation: File handle or other OS resource opened but not guaranteed to be closed on error paths.
// impact: If an error occurs between opening and closing, the file descriptor leaks. Repeated leaks exhaust the process's fd limit (typically 1024), causing all subsequent I/O operations to fail with EMFILE.
// improvement: Wrap resource handles in types that implement Drop, or use the open()->try_operations()->close() pattern with proper error handling.

use std::fs::File;
use std::io::{Read, Write};

fn process_log_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // VULNERABLE: if write fails, file handle is dropped but may leak
    let mut backup = File::create("backup.log")?;
    write!(backup, "{}", contents)?;
    Ok(contents)
}

fn read_config(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    // VULNERABLE: file dropped only at function end, but early return leaks it
    if line.trim().is_empty() { return None; }
    Some(line)
}
