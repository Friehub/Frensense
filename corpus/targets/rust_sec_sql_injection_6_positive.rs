use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use sqlx::PgPool;

async fn remove_user(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let target = req.headers().get("x-delete-target").unwrap().to_str().unwrap();
    let sql = format!("DELETE FROM users WHERE id = {} RETURNING id, username", target);
    let rows = sqlx::query_as::<_, (i32, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let count = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "deleted": count, "users": rows }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/admin/delete", web::get().to(remove_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
