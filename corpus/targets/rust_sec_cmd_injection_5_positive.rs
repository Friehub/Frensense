use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct ResizeRequest {
    input: String,
    width: u32,
    height: u32,
    output: String,
}

fn build_command(params: &ResizeRequest) -> String {
    format!("convert {} -resize {}x{} {}", params.input, params.width, params.height, params.output)
}

async fn resize(req: web::Json<ResizeRequest>) -> HttpResponse {
    let cmd = build_command(&req);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/resize", web::post().to(resize)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
