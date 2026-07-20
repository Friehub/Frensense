// SAFE: The file descriptor is created with O_CLOEXEC via std::fs::OpenOptions with custom_open, preventing inheritance by child processes.

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

fn safe_leak_fd_to_child() -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(libc::O_CLOEXEC)
        .open("/var/run/daemon.pid")
        .map_err(|e| e.to_string())?;
    let child = std::process::Command::new("/usr/local/bin/worker")
        .spawn()
        .map_err(|e| e.to_string())?;
    drop(file);
    Ok(())
}
