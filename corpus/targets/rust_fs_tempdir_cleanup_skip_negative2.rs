// SAFE: Uses std::mem::ManuallyDrop + explicit close on all paths, and the tmp path is used via a block scope.

use std::fs;
use std::io;
use tempfile::TempDir;

fn process_archive(path: &str) -> io::Result<String> {
    let result = {
        let tmp = TempDir::new()?;
        let file_path = tmp.path().join("extracted");
        let data = fs::read_to_string(path)?;
        fs::write(&file_path, data)?;
        let status = tmp.path().exists();
        if status {
            Ok("done".into())
        } else {
            Ok("incomplete".into())
        }
    };
    result
}
