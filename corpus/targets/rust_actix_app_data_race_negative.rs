use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(42u32);
    // SAFE: app_data is set before run(), not mutated after.
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("hello") }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
