// [frensense]
// observation: `.unwrap()` is called inside a function that returns `Result`, causing a panic on error instead of propagating it via `?`.
// impact: A recoverable error causes the entire application to panic and crash instead of being handled or returned to the caller.
// improvement: Use `?` to propagate the error, or handle it with `.map_err()` / pattern matching.

use std::fs;
use std::io::Read;

fn read_config(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}

fn parse_port(raw: &str) -> Result<u16, String> {
    let port: u16 = raw.parse().unwrap();
    Ok(port)
}
