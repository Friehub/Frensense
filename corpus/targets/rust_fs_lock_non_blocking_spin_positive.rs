// [frensense]
// observation: A non-blocking file lock (flock with LOCK_NB) is attempted in a busy loop without a sleep or backoff, burning 100% CPU while waiting for the lock.
// impact: Consumes an entire CPU core while spinning, causing system slowdown, higher power consumption, and starving other processes.
// improvement: Add a small sleep (e.g., std::thread::sleep) or use a blocking lock (fs2::FileExt::lock_exclusive) instead.

use fs2::FileExt;
use std::fs::OpenOptions;
use std::io;

fn lock_and_update(path: &str) -> io::Result<()> {
    let file = OpenOptions::new().write(true).open(path)?;
    loop {
        if file.try_lock_exclusive().is_ok() {
            break;
        }
    }
    Ok(())
}

fn lock_and_read(path: &str) -> io::Result<String> {
    let file = OpenOptions::new().read(true).open(path)?;
    loop {
        if file.try_lock_shared().is_ok() {
            break;
        }
    }
    std::fs::read_to_string(path)
}
