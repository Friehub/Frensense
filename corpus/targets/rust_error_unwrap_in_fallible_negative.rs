// SAFE: Uses `?` to propagate errors instead of `.unwrap()`
use std::fs;
use std::io::Read;

fn read_config(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_port(raw: &str) -> Result<u16, String> {
    let port: u16 = raw.parse().map_err(|e| format!("invalid port: {}", e))?;
    Ok(port)
}
