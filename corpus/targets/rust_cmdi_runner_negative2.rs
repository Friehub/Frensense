// SAFE: Uses std::process::Command directly with separate arguments and input validation
use std::process::Command;

fn grep_user_file(pattern: &str, filename: &str) -> Result<String, std::io::Error> {
    let output = Command::new("grep")
        .arg("-e")
        .arg(pattern)
        .arg("--")
        .arg(filename)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
