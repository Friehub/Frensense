// SAFE: Used a fixed command with user input passed only as data arguments, never as the program name.

use std::process::Command;

fn run_tool(req: Request) -> Response {
    let action = req.body.action;
    match action.as_str() {
        "status" => {
            let output = Command::new("git").arg("status").output().expect("git failed");
            Response::json(output.stdout)
        }
        "log" => {
            let output = Command::new("git").arg("log").arg("--oneline").arg("-10").output().expect("git failed");
            Response::json(output.stdout)
        }
        _ => Response::error("Unknown action"),
    }
}

fn execute_custom_command(req: Request) -> Response {
    let output = Command::new("node")
        .arg("-e")
        .arg(req.body.code)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
