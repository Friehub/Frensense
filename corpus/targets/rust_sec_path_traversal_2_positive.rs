use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;

async fn download(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let file = query.get("path").unwrap();
    let full_path = PathBuf::from("/data/public").join(file);
    match fs::read(&full_path) {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::NotFound().body("not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/download", web::post().to(download)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
