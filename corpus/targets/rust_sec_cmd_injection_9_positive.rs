use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct DeployRequest {
    environment: String,
    git_ref: String,
}

async fn deploy(req: web::Json<DeployRequest>) -> HttpResponse {
    let cmd = format!("deploy.sh --env {} --ref {}", req.environment, req.git_ref);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .current_dir("/app")
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/deploy", web::post().to(deploy)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
