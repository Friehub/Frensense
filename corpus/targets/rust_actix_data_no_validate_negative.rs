// SAFE: Validates the deserialized struct before processing
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex = "^(admin|user|moderator)$")]
    pub role: String,
}

async fn create_user(body: web::Json<CreateUserRequest>) -> HttpResponse {
    let user = body.into_inner();
    if let Err(e) = user.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": e.to_string()}));
    }
    HttpResponse::Ok().json(serde_json::json!({"created": true, "username": user.username}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/users", web::post().to(create_user)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
