// [frensense]
// observation: A file lock (`fs2::FileLock` or `std::fs::File` with `flock`) is acquired but not released on error — the lock guard is dropped only on the happy path, or the lock struct is leaked.
// impact: Resource leak — the lock remains held until the process exits, causing other processes to block indefinitely waiting for the lock to be released.
// improvement: Use RAII wrappers that always release the lock on drop, or ensure the lock is released in all code paths.

use fs2::FileExt;
use std::fs::File;

pub fn locked_write(file: &File, data: &[u8]) -> std::io::Result<()> {
    file.lock_exclusive()?;
    if data.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "empty data"));
    }
    file.write_all(data)
}
