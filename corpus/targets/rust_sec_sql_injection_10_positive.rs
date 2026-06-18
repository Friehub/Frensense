use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct BulkDelete {
    ids: Vec<i32>,
}

async fn bulk_remove(pool: web::Data<PgPool>, req: web::Json<BulkDelete>) -> HttpResponse {
    let id_list: Vec<String> = req.ids.iter().map(|id| id.to_string()).collect();
    let sql = format!("DELETE FROM items WHERE id IN ({}) RETURNING id, name", id_list.join(","));
    let rows = sqlx::query_as::<_, (i32, String)>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    let count = rows.len();
    HttpResponse::Ok().json(serde_json::json!({ "deleted": count, "items": rows }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/bulk-delete", web::post().to(bulk_remove))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
