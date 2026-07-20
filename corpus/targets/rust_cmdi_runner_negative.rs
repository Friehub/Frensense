// SAFE: Uses duct with separate arguments instead of shell string
use duct::cmd;

fn grep_user_file(pattern: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = cmd!("grep", "-e", pattern, "--", filename).read()?;
    Ok(output)
}
