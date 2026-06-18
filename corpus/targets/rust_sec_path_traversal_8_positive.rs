use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigRequest {
    section: String,
    key: String,
    value: serde_json::Value,
}

async fn save_config(req: web::Json<ConfigRequest>) -> HttpResponse {
    let config_path = PathBuf::from("/etc/app/configs")
        .join(&req.section)
        .join(format!("{}.json", req.key));
    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match fs::write(&config_path, serde_json::to_string(&req.value).unwrap()) {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "saved": config_path })),
        Err(_) => HttpResponse::InternalServerError().body("write failed"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/save-config", web::post().to(save_config)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
