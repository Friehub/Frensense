use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExportRequest {
    dir: String,
    filename: String,
}

async fn export(req: web::Json<ExportRequest>) -> HttpResponse {
    let source = PathBuf::from("/data/reports").join(&req.dir).join(&req.filename);
    let dest = PathBuf::from("/tmp/exports").join(&req.filename);
    match fs::copy(&source, &dest) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "exported": dest })),
        Err(_) => HttpResponse::NotFound().body("source not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/export", web::post().to(export)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
