// [frensense]
// observation: User input is passed directly to duct::cmd! which runs arbitrary commands via shell expansion.
// impact: An attacker can execute arbitrary system commands by injecting into the duct pipeline.
// improvement: Avoid duct with user-controlled strings; use std::process::Command with separate args or validate input.

use duct::cmd;

fn grep_user_file(pattern: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = cmd!("grep", pattern, filename).read()?;
    Ok(output)
}

fn run_pipeline(user_cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = cmd!("bash", "-c", user_cmd).read()?;
    Ok(output)
}
