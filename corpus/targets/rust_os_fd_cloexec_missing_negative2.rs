// SAFE: Uses fcntl to set FD_CLOEXEC on the file descriptor after creation, ensuring it is not inherited across exec.

use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};

fn safe_open_and_exec() -> Result<(), String> {
    let file = File::create("/var/log/app.log").map_err(|e| e.to_string())?;
    let fd = file.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        if flags == -1 {
            return Err("fcntl failed".into());
        }
        let ret = libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC);
        if ret == -1 {
            return Err("fcntl set failed".into());
        }
    }
    let child = std::process::Command::new("logger")
        .arg("--fd")
        .arg(fd.to_string())
        .spawn()
        .map_err(|e| e.to_string())?;
    child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(())
}
