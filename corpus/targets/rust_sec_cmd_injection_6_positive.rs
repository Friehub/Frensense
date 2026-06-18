use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};

async fn proxy(req: HttpRequest) -> HttpResponse {
    let url = req.headers().get("x-target-url").unwrap().to_str().unwrap();
    let cmd = format!("curl -s \"{}\"", url);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/proxy", web::get().to(proxy)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
