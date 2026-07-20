// [frensense]
// observation: Sensitive data placed in HttpRequest extensions is returned directly in the response body, leaking internal state to the caller.
// impact: Internal authentication tokens, user roles, or database handles stored in request extensions become visible in HTTP responses.
// improvement: Avoid serializing request extensions into responses; redact or strip extension data before returning.

use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct SessionInfo {
    user_id: String,
    token: String,
    role: String,
}

async fn profile(req: HttpRequest) -> HttpResponse {
    if let Some(session) = req.extensions().get::<SessionInfo>() {
        HttpResponse::Ok().json(session)
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
