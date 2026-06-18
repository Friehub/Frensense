use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProfileUpdate {
    bio: String,
    location: String,
}

async fn update_profile(pool: web::Data<PgPool>, req: HttpRequest, body: web::Json<ProfileUpdate>) -> HttpResponse {
    let user_id = req.headers().get("x-user-id").unwrap().to_str().unwrap();
    let sql = format!(
        "UPDATE profiles SET bio = '{}', location = '{}', updated_at = NOW() WHERE user_id = {} RETURNING *",
        body.bio, body.location, user_id
    );
    let row = sqlx::query_as::<_, (i32, String, String)>(&sql)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "profile": row, "updated": true }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://localhost/db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/update-profile", web::post().to(update_profile))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
