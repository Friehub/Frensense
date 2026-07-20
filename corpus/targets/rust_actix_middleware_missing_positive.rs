// [frensense]
// observation: A route is registered in Actix-web without any authentication middleware, exposing sensitive endpoints to unauthenticated callers.
// impact: An unauthenticated attacker can access admin panels, user data, or perform privileged actions.
// improvement: Apply a middleware or guard that checks authentication before the handler runs.

use actix_web::{web, App, HttpServer, HttpResponse};

async fn admin_dashboard() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"users": ["alice", "bob"], "secrets": "supersecret"}))
}

async fn delete_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({"deleted": user_id}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/admin", web::get().to(admin_dashboard))
            .route("/admin/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
