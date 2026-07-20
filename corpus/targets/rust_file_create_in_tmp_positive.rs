// [frensense]
// observation: A temporary file is created in `/tmp/` with a predictable name derived from user input, making it vulnerable to symlink attacks or race conditions.
// impact: An attacker can pre-create the target file as a symlink to an arbitrary file, causing the application to overwrite or read sensitive files when the temp file is written.
// improvement: Use `tempfile::NamedTempFile` or include a random component in the filename.

use std::fs;
use std::path::Path;

fn write_temp_cache(user_id: String, data: &[u8]) -> std::io::Result<()> {
    let path = format!("/tmp/cache_{}", user_id);
    fs::write(&path, data)?;
    Ok(())
}

fn read_temp_report(report_id: String) -> std::io::Result<String> {
    let path = format!("/tmp/report_{}.html", report_id);
    fs::read_to_string(&path)
}
