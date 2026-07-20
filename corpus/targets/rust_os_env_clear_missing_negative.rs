// SAFE: .env_clear() is called before setting specific environment variables, preventing parent env var leakage to child processes.

use std::process::Command;

fn run_backup_script(path: &str) -> Result<(), String> {
    let status = Command::new("/usr/local/bin/backup")
        .env_clear()
        .env("BACKUP_DIR", "/var/backups")
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
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin")
        .output()
        .map_err(|e| e.to_string())?;
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
