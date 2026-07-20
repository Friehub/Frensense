// SAFE: If non-blocking is required, a sleep with exponential backoff prevents CPU burning while waiting.

use fs2::FileExt;
use std::fs::OpenOptions;
use std::io;
use std::time::Duration;

fn lock_and_update(path: &str) -> io::Result<()> {
    let file = OpenOptions::new().write(true).open(path)?;
    let mut delay = Duration::from_millis(10);
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => return Ok(()),
            Err(_) => {
                std::thread::sleep(delay);
                if delay < Duration::from_secs(1) {
                    delay *= 2;
                }
            }
        }
    }
}
