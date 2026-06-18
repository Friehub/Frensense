use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};

async fn search(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = query.get("q").unwrap();
    let cmd = format!("grep -r \"{}\" /data/", q);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute");
    HttpResponse::Ok().body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/search", web::get().to(search)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
