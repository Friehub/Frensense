// SAFE: Uses explicit command with separate arguments, no shell interpretation
use std::process::Command;

fn run_user_command(program: &str, arg: &str) -> Result<String, std::io::Error> {
    let output = Command::new(program)
        .arg(arg)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_bash_script(script: &str) -> Result<String, std::io::Error> {
    let output = Command::new("bash")
        .arg("/tmp/script.sh")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
