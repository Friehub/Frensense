use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;

async fn get_orders(pool: web::Data<PgPool>, path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let sql = format!("SELECT o.id, o.total, o.created_at FROM orders o WHERE o.user_id = {} AND o.status = 'completed' ORDER BY o.created_at DESC LIMIT 50", user_id);
    let rows = sqlx::query_as::<_, (i32, f64, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let count = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "orders": rows, "count": count }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/report/{id}", web::get().to(get_orders))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
