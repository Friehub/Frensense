use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};

async fn stats(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let metric = query.get("metric").unwrap();
    let period = query.get("period").unwrap();
    let cmd = format!("collectd --query \"{}\" --since \"{}\"", metric, period);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(String::from_utf8_lossy(&output.stdout).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/stats", web::get().to(stats)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
