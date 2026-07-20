// SAFE: Validates input against allowed characters before passing to shell
fn run_command(input: &str) -> String {
    let sanitized: String = input.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect();
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", sanitized))
        .output()
        .expect("failed");
    String::from_utf8_lossy(&output.stdout).to_string()
}
