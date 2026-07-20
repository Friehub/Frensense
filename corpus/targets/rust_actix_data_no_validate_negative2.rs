// SAFE: Uses a manual guard function to validate fields after extraction
use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub role: String,
}

fn validate_request(req: &CreateUserRequest) -> Result<(), String> {
    if req.username.len() < 3 || req.username.len() > 32 {
        return Err("invalid username length".into());
    }
    if !req.email.contains('@') {
        return Err("invalid email".into());
    }
    match req.role.as_str() {
        "admin" | "user" | "moderator" => Ok(()),
        _ => Err("invalid role".into()),
    }
}

async fn create_user(body: web::Json<CreateUserRequest>) -> HttpResponse {
    let user = body.into_inner();
    if let Err(msg) = validate_request(&user) {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": msg}));
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
