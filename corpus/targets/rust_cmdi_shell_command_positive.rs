// [frensense]
// observation: User-controlled input is passed to sh -c or equivalent via Command::new("sh").arg("-c"), allowing arbitrary command injection through shell metacharacters.
// impact: An attacker can inject shell metacharacters (;, &, |, $(), ``) to execute arbitrary commands on the server with the process's privileges.
// improvement: Avoid using shell invocation with user input; use direct commands with separate arguments instead.

use std::process::Command;

fn run_command(req: Request) -> Response {
    let user_input = req.body.command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo 'Result: {}'", user_input))
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn cleanup_logs(req: Request) -> Response {
    let pattern = req.body.pattern;
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("rm -f /var/log/{}", pattern))
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
