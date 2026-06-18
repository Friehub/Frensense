use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Serialize)]
struct UserSummary {
    id: i32,
    display_name: String,
    contact_email: String,
}

async fn find_users(pool: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let name = query.get("name").unwrap();
    let sql = format!("SELECT id, name, email FROM users WHERE name = '{}' AND active = true", name);
    let rows = sqlx::query_as::<_, (i32, String, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let users: Vec<UserSummary> = rows.into_iter().map(|(id, name, email)| {
        UserSummary { id, display_name: name, contact_email: email }
    }).collect();
    let count = users.len();
    HttpResponse::Ok().json(serde_json::json!({ "users": users, "count": count }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(find_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
