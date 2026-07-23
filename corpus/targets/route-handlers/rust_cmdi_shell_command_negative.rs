// SAFE: Removed shell invocation and used direct command with separate arguments, preventing shell injection.

use std::process::Command;

fn run_command(req: Request) -> Response {
    let user_input = req.body.command;
    let output = Command::new("echo")
        .arg("Result:")
        .arg(&user_input)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn cleanup_logs(req: Request) -> Response {
    let pattern = req.body.pattern;
    let output = Command::new("find")
        .arg("/var/log")
        .arg("-name")
        .arg(&pattern)
        .arg("-delete")
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
