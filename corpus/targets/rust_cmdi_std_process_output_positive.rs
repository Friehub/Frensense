// [frensense]
// observation: User input is passed to sh -c via Command::new("sh").arg("-c").arg(user_input), allowing arbitrary shell command execution.
// impact: An attacker can execute arbitrary shell commands by injecting shell metacharacters like ;, |, or $(...).
// improvement: Avoid shell wrappers; use Command::arg with separate arguments, or validate input against a strict allowlist.

use std::process::Command;

fn run_user_command(cmd: &str) -> Result<String, std::io::Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_bash_script(script: &str) -> Result<String, std::io::Error> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
