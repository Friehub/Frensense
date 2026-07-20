// SAFE: Used Command::args with fixed positional arguments; user input is prefixed with a base directory to prevent flag injection.

use std::path::PathBuf;
use std::process::Command;

fn read_file(req: Request) -> Response {
    let filename = req.body.filename;
    let base = PathBuf::from("/data");
    let full_path = base.join(filename);
    if !full_path.starts_with(&base) {
        return Response::error("Path traversal detected");
    }
    let output = Command::new("cat")
        .arg(full_path.to_str().unwrap())
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn process_files(req: Request) -> Response {
    let input = req.body.input;
    let output_path = req.body.output;
    let input_path = format!("/input/{}", input.replace("..", "").replace('/', ""));
    let output_path = format!("/output/{}", output_path.replace("..", "").replace('/', ""));
    let output = Command::new("convert")
        .arg(input_path)
        .arg("-resize")
        .arg("800x800")
        .arg(output_path)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
