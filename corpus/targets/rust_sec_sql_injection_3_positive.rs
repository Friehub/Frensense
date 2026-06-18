use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;

async fn dynamic_search(pool: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let field = query.get("field").unwrap();
    let value = query.get("value").unwrap();
    let sort_by = query.get("sort").map(|s| s.as_str()).unwrap_or("created_at");
    let sql = format!("SELECT * FROM products WHERE {} = '{}' ORDER BY {} DESC", field, value, sort_by);
    let rows = sqlx::query_as::<_, (i32, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let total = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "results": rows, "total": total }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/search", web::get().to(dynamic_search))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
