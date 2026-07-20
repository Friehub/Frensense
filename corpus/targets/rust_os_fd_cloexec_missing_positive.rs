// [frensense]
// observation: A file descriptor is opened without the O_CLOEXEC flag and then the process forks/execs a child, leaking the fd to the child process.
// impact: The child process inherits open file descriptors, potentially gaining access to sensitive files, sockets, or locks that should be private to the parent.
// improvement: Use O_CLOEXEC flag when opening files, or call fcntl(fd, F_SETFD, FD_CLOEXEC) before exec.

use std::fs::File;
use std::os::unix::io::AsRawFd;

fn leak_fd_to_child() -> Result<(), String> {
    let file = File::create("/var/run/daemon.pid").map_err(|e| e.to_string())?;
    // fd is inherited by child
    let child = std::process::Command::new("/usr/local/bin/worker")
        .spawn()
        .map_err(|e| e.to_string())?;
    drop(file);
    Ok(())
}

fn open_log_and_exec() -> Result<(), String> {
    let log = File::create("/var/log/app.log").map_err(|e| e.to_string())?;
    let fd = log.as_raw_fd();
    let child = std::process::Command::new("logger")
        .arg("--fd")
        .arg(fd.to_string())
        .spawn()
        .map_err(|e| e.to_string())?;
    child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(())
}
