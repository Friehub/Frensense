// SAFE: Rejects any entry whose name contains ../ or absolute path components
use std::fs;
use std::path::Path;
use zip::ZipArchive;

fn extract_archive(archive_path: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let name = entry.name().replace('\\', "/");
        if name.contains("..") || Path::new(&name).is_absolute() {
            eprintln!("Skipping unsafe entry: {}", name);
            continue;
        }
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
