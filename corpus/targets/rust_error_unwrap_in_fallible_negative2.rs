// SAFE: Handles errors with match or if-let instead of unwrap
use std::fs;
use std::io::Read;

fn read_config(path: &str) -> Result<String, String> {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("cannot open: {}", e)),
    };
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        return Err(format!("cannot read: {}", e));
    }
    Ok(content)
}

fn parse_port(raw: &str) -> Result<u16, String> {
    match raw.parse() {
        Ok(p) if p > 0 => Ok(p),
        Ok(_) => Err("port must be positive".into()),
        Err(e) => Err(format!("invalid port: {}", e)),
    }
}
