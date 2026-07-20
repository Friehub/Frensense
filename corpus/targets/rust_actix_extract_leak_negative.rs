// SAFE: Only non-sensitive fields from the extension are mapped to a response DTO
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use serde::Serialize;

#[derive(Clone)]
struct SessionInfo {
    user_id: String,
    token: String,
    role: String,
}

#[derive(Serialize)]
struct ProfileResponse {
    user_id: String,
    role: String,
}

async fn profile(req: HttpRequest) -> HttpResponse {
    if let Some(session) = req.extensions().get::<SessionInfo>() {
        let resp = ProfileResponse {
            user_id: session.user_id.clone(),
            role: session.role.clone(),
        };
        HttpResponse::Ok().json(resp)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/profile", web::get().to(profile)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
