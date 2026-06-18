use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;

async fn export_table(pool: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
    let table = query.get("table").unwrap();
    let where_clause = query.get("where").map(|s| s.as_str()).unwrap_or("1=1");
    let sql = format!("SELECT * FROM {} WHERE {} ORDER BY id LIMIT 1000", table, where_clause);
    let rows = sqlx::query_as::<_, (i32, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let count = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "data": rows, "count": count }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/export", web::get().to(export_table))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
