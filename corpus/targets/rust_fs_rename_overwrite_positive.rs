// [frensense]
// observation: `std::fs::rename` is called without checking whether the destination path already exists. On Unix, rename atomically overwrites the destination, silently destroying data.
// impact: Silent data loss — the destination file is replaced without warning. If the destination is a configuration file or database, critical data is lost.
// improvement: Check if the destination exists before renaming, or use `rename` only when overwrite is intended and acceptable.

use std::fs;
use std::path::Path;

pub fn move_file(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::rename(src, dst)
}
