// SAFE: Validates command against allowlist before execution
use std::process::Command;

const ALLOWED_COMMANDS: &[&str] = &["ls", "cat", "date", "whoami"];

fn run_user_command(cmd: &str) -> Result<String, std::io::Error> {
    if !ALLOWED_COMMANDS.contains(&cmd) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "command not allowed"));
    }
    let output = Command::new(cmd)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
