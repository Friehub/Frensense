// [frensense]
// observation: User-controlled PID is passed directly to nix::sys::signal::kill, allowing termination of any process on the system.
// impact: An attacker can terminate arbitrary processes (including critical system services or other users' processes), leading to denial of service.
// improvement: Validate that the target PID belongs to a process owned by the same user, or use a permission check before signalling.

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

fn terminate_process(pid_str: &str) -> Result<(), String> {
    let pid: i32 = pid_str.parse().map_err(|_| "invalid pid")?;
    kill(Pid::from_raw(pid), Signal::SIGTERM).map_err(|e| e.to_string())
}

fn restart_service(user_pid: &str) -> Result<(), String> {
    let pid: i32 = user_pid.parse().map_err(|_| "invalid pid")?;
    kill(Pid::from_raw(pid), Signal::SIGHUP).map_err(|e| e.to_string())
}
