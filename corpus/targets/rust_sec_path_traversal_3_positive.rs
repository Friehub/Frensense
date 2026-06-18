use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct BackupRequest {
    destination: String,
}

async fn backup(req: web::Json<BackupRequest>) -> HttpResponse {
    let target = PathBuf::from("/backups").join(&req.destination);
    match fs::copy("/data/db.sqlite", &target) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "backed_up": target })),
        Err(_) => HttpResponse::InternalServerError().body("backup failed"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/backup", web::post().to(backup)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
