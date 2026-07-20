// [frensense]
// observation: std::process::Command is used to spawn a child process without calling .env_clear(), inheriting all environment variables from the parent.
// impact: Sensitive environment variables (API keys, database passwords, tokens) are leaked to child processes, potentially crossing privilege boundaries or being exposed via /proc.
// improvement: Call .env_clear() before .env() to explicitly control which variables are passed to child processes.

use std::process::Command;

fn run_backup_script(path: &str) -> Result<(), String> {
    let status = Command::new("/usr/local/bin/backup")
        .arg(path)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("backup failed".into());
    }
    Ok(())
}

fn execute_plugin(path: &str) -> Result<String, String> {
    let output = Command::new(path)
        .output()
        .map_err(|e| e.to_string())?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
