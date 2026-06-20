fn run_command(input: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", input))
        .output()
        .expect("failed");
    String::from_utf8_lossy(&output.stdout).to_string()
}
