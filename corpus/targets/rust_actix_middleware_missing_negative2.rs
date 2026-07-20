// SAFE: Uses actix-web's HttpAuthentication middleware from actix-web-httpauth
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

async fn bearer_validator(req: actix_web::HttpRequest, credentials: BearerAuth) -> Result<BearerAuth, actix_web::Error> {
    let token = credentials.token();
    if token != "valid-secret-token" {
        let config = req.app_data::<Config>().cloned().unwrap_or_default();
        return Err(AuthenticationError::from(config).into());
    }
    Ok(credentials)
}

async fn admin_dashboard() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"users": ["alice", "bob"]}))
}

async fn delete_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({"deleted": user_id}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(bearer_validator);
        App::new()
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .route("", web::get().to(admin_dashboard))
                    .route("/users/{id}", web::delete().to(delete_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
