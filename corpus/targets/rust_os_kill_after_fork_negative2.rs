// SAFE: Uses capability-based process signalling via process group ownership, preventing signalling of non-child processes.

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;

fn run_worker() -> Result<(), String> {
    let child = std::process::Command::new("/usr/local/bin/worker")
        .process_group(0)
        .spawn()
        .map_err(|e| e.to_string())?;
    let pid = Pid::from_raw(child.id() as i32);
    kill(pid, Signal::SIGTERM).map_err(|e| e.to_string())
}
