use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;

async fn find_user(pool: web::Data<PgPool>, identifier: &str) -> Vec<(i32, String, String, String)> {
    let sql = format!(
        "SELECT id, username, email, role FROM users WHERE email = '{}' OR username = '{}' AND active = true",
        identifier, identifier
    );
    sqlx::query_as::<_, (i32, String, String, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap()
}

async fn lookup(pool: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let q = query.get("q").unwrap();
    let users = find_user(pool.as_ref(), q).await;
    let found = !users.is_empty();
    HttpResponse::Ok().json(serde_json::json!({ "users": users, "found": found }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/lookup", web::get().to(lookup))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
