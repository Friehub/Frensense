// SAFE: Includes a random hex suffix in the filename to prevent predictability
use std::fs;
use rand::Rng;

fn random_suffix() -> String {
    let mut rng = rand::thread_rng();
    format!("{:016x}", rng.gen::<u64>())
}

fn write_temp_cache(user_id: String, data: &[u8]) -> std::io::Result<String> {
    let suffix = random_suffix();
    let path = format!("/tmp/cache_{}_{}", user_id, suffix);
    fs::write(&path, data)?;
    Ok(path)
}

fn read_temp_report(report_id: String) -> Result<String, String> {
    let suffix = random_suffix();
    let path = format!("/tmp/report_{}_{}.html", report_id, suffix);
    fs::read_to_string(&path).map_err(|e| e.to_string())
}
