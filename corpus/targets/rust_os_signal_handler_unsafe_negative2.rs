// SAFE: Uses signalfd to handle signals synchronously in an event loop, completely avoiding signal handler context restrictions.

use std::os::fd::AsRawFd;
use std::io::Read;

fn signal_event_loop() -> Result<(), String> {
    let mut sigset = nix::sys::signal::SigSet::empty();
    sigset.add(nix::sys::signal::SIGTERM);
    sigset.add(nix::sys::signal::SIGINT);
    sigset.thread_block().map_err(|e| e.to_string())?;

    let sfd = nix::sys::signalfd::SignalFd::new(&sigset)
        .map_err(|e| e.to_string())?;

    loop {
        let _ = sfd.read_signal().map_err(|e| e.to_string())?;
        // safe to allocate here — not in signal handler context
        let _log = format!("signal received");
    }
}
