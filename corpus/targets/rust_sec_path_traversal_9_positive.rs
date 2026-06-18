use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;

async fn read_doc(name: &str) -> Result<String, std::io::Error> {
    let full_path = PathBuf::from("/data/docs").join(name);
    fs::read_to_string(full_path)
}

async fn get_doc(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = query.get("name").unwrap();
    match read_doc(name).await {
        Ok(content) => HttpResponse::Ok().content_type("text/markdown").body(content),
        Err(_) => HttpResponse::NotFound().body("document not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/doc", web::get().to(get_doc)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
