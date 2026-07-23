// [frensense]
// observation: User-controlled input is passed as an argument to Command::arg without validation, allowing argument injection via flag-like values.
// impact: An attacker can inject arguments that alter the command's behavior (e.g., --exec, -o, --output) to read or write arbitrary files on the system.
// improvement: Validate user-supplied arguments against an allowlist or use a wrapper that prevents argument injection.

use std::process::Command;

fn read_file(req: Request) -> Response {
    let filename = req.body.filename;
    let output = Command::new("cat")
        .arg(filename)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn process_files(req: Request) -> Response {
    let input = req.body.input;
    let output_path = req.body.output;
    let output = Command::new("convert")
        .arg(input)
        .arg("-resize")
        .arg("800x800")
        .arg(output_path)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
