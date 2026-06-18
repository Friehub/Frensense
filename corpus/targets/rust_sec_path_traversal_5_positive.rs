use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use std::path::PathBuf;

async fn get_icon(req: HttpRequest) -> HttpResponse {
    let icon = req.headers().get("x-icon-name").unwrap().to_str().unwrap();
    let icon_path = PathBuf::from("/static/icons").join(icon);
    HttpResponse::Ok().body(icon_path.to_string_lossy().as_bytes())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/icon", web::get().to(get_icon)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
