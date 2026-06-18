use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct LogQuery {
    level: String,
    source: String,
    limit: Option<i32>,
}

async fn query_logs(pool: web::Data<PgPool>, query: web::Query<LogQuery>) -> HttpResponse {
    let limit = query.limit.unwrap_or(100);
    let sql = format!(
        "SELECT timestamp, message, metadata FROM logs WHERE level = '{}' AND source = '{}' ORDER BY timestamp DESC LIMIT {}",
        query.level, query.source, limit
    );
    let rows = sqlx::query_as::<_, (String, String, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let count = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "logs": rows, "count": count }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/logs", web::get().to(query_logs))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
