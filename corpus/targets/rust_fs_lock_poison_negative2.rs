// SAFE: LockGuard RAII wrapper ensures lock release on any exit path.
use std::fs::File;
use std::io::Write;

struct LockGuard<'a>(&'a File);

impl<'a> LockGuard<'a> {
    fn new(file: &'a File) -> std::io::Result<Self> {
        file.lock_exclusive()?;
        Ok(LockGuard(file))
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        self.0.unlock().ok();
    }
}

pub fn locked_write(file: &File, data: &[u8]) -> std::io::Result<()> {
    let _guard = LockGuard::new(file)?;
    if data.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "empty data"));
    }
    file.write_all(data)
}
