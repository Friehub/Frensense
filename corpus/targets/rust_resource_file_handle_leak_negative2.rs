// SAFE alternative: explicit scope management
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_config(path: &str) -> Option<String> {
    let result = (|| -> Option<String> {
        let file = File::open(path).ok()?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        if line.trim().is_empty() { return None; }
        Some(line)
    })();
    // All temporaries dropped at the end of the closure
    result
}
