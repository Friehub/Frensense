use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;

async fn get_log(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let log_file = query.get("file").unwrap();
    let full_path = PathBuf::from("/var/log/app").join(log_file);
    match fs::read_to_string(&full_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/plain").body(content),
        Err(_) => HttpResponse::NotFound().body("log not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/log", web::get().to(get_log)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
