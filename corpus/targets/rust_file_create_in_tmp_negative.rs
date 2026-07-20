// SAFE: Uses `tempfile::NamedTempFile` to create a unique temporary file with unpredictable name
use std::io::Write;

fn write_temp_cache(user_id: String, data: &[u8]) -> Result<String, std::io::Error> {
    let mut tmp = tempfile::NamedTempFile::new()?;
    tmp.write_all(data)?;
    let (file, path) = tmp.keep()?;
    drop(file);
    let final_path = format!("/tmp/cache_{}_{}", user_id, path.file_name().unwrap().to_string_lossy());
    std::fs::rename(&path, &final_path)?;
    Ok(final_path)
}
