use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;

async fn get_file(path: web::Path<String>) -> HttpResponse {
    let filename = path.into_inner();
    let full_path = PathBuf::from("/data/uploads").join(&filename);
    match fs::read_to_string(&full_path) {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::NotFound().body("not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/files/{filename}", web::get().to(get_file)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
