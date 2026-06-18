use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct PingRequest {
    host: String,
}

async fn ping(req: web::Json<PingRequest>) -> HttpResponse {
    let output = Command::new("ping")
        .args(["-c", "3", &req.host])
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ping", web::post().to(ping)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
