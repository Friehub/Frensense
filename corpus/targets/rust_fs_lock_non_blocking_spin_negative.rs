// SAFE: Uses blocking lock instead of spin-looping, yielding the CPU to the kernel while waiting for the lock.

use fs2::FileExt;
use std::fs::OpenOptions;
use std::io;

fn lock_and_update(path: &str) -> io::Result<()> {
    let file = OpenOptions::new().write(true).open(path)?;
    file.lock_exclusive()?;
    Ok(())
}

fn lock_and_read(path: &str) -> io::Result<String> {
    let file = OpenOptions::new().read(true).open(path)?;
    file.lock_shared()?;
    std::fs::read_to_string(path)
}
