// SAFE: Rust's Drop trait ensures file handles are closed even on error
use std::fs::File;
use std::io::{Read, Write};

fn process_log_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut backup = File::create("backup.log")?;
    write!(backup, "{}", contents)?;
    // file is dropped here when it goes out of scope
    Ok(contents)
}

fn read_config(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    if line.trim().is_empty() { return None; }
    Some(line)
}
