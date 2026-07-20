// SAFE: Backs up destination before overwriting with rename.
use std::fs;
use std::path::Path;

pub fn move_file_with_backup(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        let backup = dst.with_extension("bak");
        fs::rename(dst, &backup)?;
    }
    fs::rename(src, dst)
}
