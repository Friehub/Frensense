// [frensense]
// observation: Archive entry file names are used directly in path joins during extraction without checking for ../ traversal components, enabling zip-slip.
// impact: An attacker can craft a zip archive with entries containing ../ to overwrite arbitrary files outside the extraction directory.
// improvement: Validate each entry path by canonicalizing and checking it stays within the target directory.

use std::fs;
use std::path::Path;
use zip::ZipArchive;
use std::io::Read;

fn extract_archive(archive_path: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.name().to_string();
        let outpath = Path::new(dest).join(&name);
        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }
    Ok(())
}
