use std::fs::File;
use std::io::Write;

pub fn locked_write(file: &File, data: &[u8]) -> std::io::Result<()> {
    let _lock = file.lock_exclusive()?;
    if data.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "empty data"));
    }
    file.write_all(data)
}
