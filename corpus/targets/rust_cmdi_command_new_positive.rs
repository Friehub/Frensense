// [frensense]
// observation: User input is passed as the program name to Command::new, allowing execution of arbitrary binaries.
// impact: An attacker can specify any executable on the system (e.g., /bin/sh, /bin/rm) to be run with the provided arguments, leading to full system compromise.
// improvement: Validate the command name against an allowlist of permitted binaries before calling Command::new.

use std::process::Command;

fn run_tool(req: Request) -> Response {
    let tool_name = req.body.tool_name;
    let output = Command::new(tool_name)
        .arg("--version")
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn execute_custom_command(req: Request) -> Response {
    let program = req.query.get("program").unwrap();
    let args: Vec<&str> = req.query.get("args").unwrap().split(' ').collect();
    let output = Command::new(program)
        .args(&args)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
