// SAFE: Validated the command name against an allowlist of permitted binaries before execution.

use std::collections::HashSet;
use std::process::Command;

fn get_allowed_commands() -> HashSet<&'static str> {
    let mut set = HashSet::new();
    set.insert("git");
    set.insert("curl");
    set.insert("node");
    set.insert("python3");
    set.insert("ls");
    set
}

fn run_tool(req: Request) -> Response {
    let tool_name = req.body.tool_name;
    let allowed = get_allowed_commands();
    if !allowed.contains(tool_name.as_str()) {
        return Response::error("Command not allowed");
    }
    let output = Command::new(tool_name)
        .arg("--version")
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn execute_custom_command(req: Request) -> Response {
    let program = req.query.get("program").unwrap();
    let allowed = get_allowed_commands();
    if !allowed.contains(program.as_str()) {
        return Response::error("Command not allowed");
    }
    let output = Command::new(program)
        .arg("--help")
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
