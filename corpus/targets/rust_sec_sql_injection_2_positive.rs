use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResult {
    id: i32,
    username: String,
    role: String,
}

async fn authenticate(pool: web::Data<PgPool>, req: web::Json<LoginRequest>) -> HttpResponse {
    let sql = format!(
        "SELECT id, username, role FROM accounts WHERE username = '{}' AND password = '{}' AND locked = false",
        req.username, req.password
    );
    let rows = sqlx::query_as::<_, (i32, String, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    if rows.is_empty() {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "invalid credentials" }));
    }
    let (id, username, role) = &rows[0];
    HttpResponse::Ok().json(serde_json::json!({
        "token": format!("session-{}", id),
        "user": AuthResult { id: *id, username: username.clone(), role: role.clone() }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/login", web::post().to(authenticate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
