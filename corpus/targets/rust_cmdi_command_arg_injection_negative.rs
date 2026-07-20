// SAFE: Validated user-supplied arguments against a strict filename pattern, preventing argument injection.

use std::process::Command;

fn is_safe_filename(name: &str) -> bool {
    name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

fn read_file(req: Request) -> Response {
    let filename = req.body.filename;
    if !is_safe_filename(&filename) {
        return Response::error("Invalid filename");
    }
    let path = format!("/data/{}", filename);
    let output = Command::new("cat")
        .arg(&path)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}

fn process_files(req: Request) -> Response {
    let input = req.body.input;
    let output_path = req.body.output;
    if !is_safe_filename(&input) || !is_safe_filename(&output_path) {
        return Response::error("Invalid filename");
    }
    let input_path = format!("/input/{}", input);
    let output_path = format!("/output/{}", output_path);
    let output = Command::new("convert")
        .arg(&input_path)
        .arg("-resize")
        .arg("800x800")
        .arg(&output_path)
        .output()
        .expect("failed to execute");
    Response::json(output.stdout)
}
