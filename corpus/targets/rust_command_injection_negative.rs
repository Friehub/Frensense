fn run_command(input: &str) -> String {
    let output = Command::new("echo")
        .arg(input)
        .output()
        .expect("failed");
    String::from_utf8_lossy(&output.stdout).to_string()
}
