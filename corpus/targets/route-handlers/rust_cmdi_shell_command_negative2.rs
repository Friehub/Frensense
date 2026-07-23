// SAFE: Used std::fs and glob crate instead of shell commands, avoiding command injection entirely.

use std::fs;
use glob::glob;

fn run_command(req: Request) -> Response {
    let message = format!("Result: {}", req.body.command);
    Response::json(message)
}

fn cleanup_logs(req: Request) -> Response {
    let pattern = format!("/var/log/{}", req.body.pattern);
    for entry in glob(&pattern).expect("invalid pattern") {
        match entry {
            Ok(path) => {
                fs::remove_file(&path).ok();
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    Response::json("Cleaned up")
}
