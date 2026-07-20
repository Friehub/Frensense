// [frensense]
// observation: Actix-web uses `web::Json<T>` or `web::Form<T>` as an extractor without any validation of the deserialized struct fields, accepting arbitrary data.
// impact: An attacker can send malformed or malicious payloads that pass type-level deserialization but contain semantically invalid or dangerous values, leading to data corruption or logic bypass.
// improvement: Add a validation step after extraction (e.g. using a `Validate` impl or guard function) before processing the data.

use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_admin: bool,
}

async fn create_user(body: web::Json<CreateUserRequest>) -> HttpResponse {
    let user = body.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "created": true,
        "username": user.username,
        "role": user.role,
        "is_admin": user.is_admin,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/users", web::post().to(create_user)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
