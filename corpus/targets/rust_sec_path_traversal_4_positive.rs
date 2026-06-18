use actix_web::{web, App, HttpServer, HttpResponse};
use std::fs;
use std::path::PathBuf;

fn resolve_template(name: &str) -> PathBuf {
    PathBuf::from("/templates").join(name)
}

async fn get_template(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let template_path = resolve_template(query.get("name").unwrap());
    match fs::read_to_string(&template_path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::NotFound().body("template not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/template", web::get().to(get_template)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
