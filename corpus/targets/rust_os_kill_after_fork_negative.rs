// SAFE: The process only kills its own child processes by tracking PIDs from fork, preventing arbitrary process termination.

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::collections::HashSet;

struct ChildProcesses {
    children: HashSet<Pid>,
}

impl ChildProcesses {
    fn terminate_child(&self, pid: Pid) -> Result<(), String> {
        if !self.children.contains(&pid) {
            return Err("not a child process".into());
        }
        kill(pid, Signal::SIGTERM).map_err(|e| e.to_string())
    }
}

fn spawn_child() -> ChildProcesses {
    let mut children = HashSet::new();
    match nix::unistd::fork() {
        Ok(nix::unistd::ForkResult::Child) => {}
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            children.insert(child);
        }
        Err(_) => {}
    }
    ChildProcesses { children }
}
