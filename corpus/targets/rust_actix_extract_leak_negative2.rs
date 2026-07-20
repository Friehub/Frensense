// SAFE: Uses a custom extractor that exposes only safe fields from the session
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use std::future::{ready, Ready};
use serde::Serialize;

#[derive(Clone)]
struct SessionInfo {
    user_id: String,
    token: String,
    role: String,
}

#[derive(Serialize)]
struct SafeSession {
    user_id: String,
    role: String,
}

impl FromRequest for SafeSession {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.extensions().get::<SessionInfo>().cloned();
        match session {
            Some(s) => ready(Ok(SafeSession { user_id: s.user_id, role: s.role })),
            None => ready(Err(actix_web::error::ErrorUnauthorized("not authenticated"))),
        }
    }
}

async fn profile(session: SafeSession) -> HttpResponse {
    HttpResponse::Ok().json(session)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/profile", web::get().to(profile)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
