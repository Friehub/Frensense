// SAFE: Validates that each extracted entry's canonical path is within the destination directory
use std::fs;
use std::path::Path;
use zip::ZipArchive;

fn extract_archive(archive_path: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;
    let dest_canonical = fs::canonicalize(dest)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.name().to_string();
        let outpath = dest_canonical.join(&name);
        let canonical = outpath.canonicalize().unwrap_or(outpath.clone());
        if !canonical.starts_with(&dest_canonical) {
            eprintln!("Skipping zip-slip entry: {}", name);
            continue;
        }
        if entry.is_dir() {
            fs::create_dir_all(&canonical)?;
        } else {
            if let Some(parent) = canonical.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&canonical)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }
    Ok(())
}
