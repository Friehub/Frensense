use std::process::Command;
use actix_web::{web, App, HttpServer, HttpResponse};

async fn backup(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let filename = query.get("file").unwrap();
    let cmd = format!("tar czf /tmp/backup.tar.gz {}", filename);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output();
    match output {
        Ok(_) => HttpResponse::Ok().body("backup created"),
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/backup", web::get().to(backup)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
