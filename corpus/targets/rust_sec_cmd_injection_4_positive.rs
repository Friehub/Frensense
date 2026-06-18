use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct ProcessRequest {
    name: String,
    action: String,
}

async fn process(req: web::Json<ProcessRequest>) -> HttpResponse {
    let cmd = format!("./worker.sh {} {}", req.action, req.name);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/process", web::post().to(process)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
