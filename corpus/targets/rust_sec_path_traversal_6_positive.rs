use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct UploadRequest {
    folder: String,
    filename: String,
    content: String,
}

async fn upload(req: web::Json<UploadRequest>) -> HttpResponse {
    let dest = PathBuf::from("/data/uploads").join(&req.folder).join(&req.filename);
    match fs::write(&dest, &req.content) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "saved": dest })),
        Err(_) => HttpResponse::InternalServerError().body("write failed"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/upload", web::post().to(upload)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
