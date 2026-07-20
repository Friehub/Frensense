// [frensense]
// observation: Actix `App::app_data()` is called after the server has started or from multiple threads, modifying shared application state without synchronization. The `app_data` registry is not designed for mutation after the server begins handling requests.
// impact: Data race on the application state registry. Concurrent reads from request handlers and a late mutation can cause a panic (due to type-map internal mutation), undefined behavior, or inconsistent state across workers, leading to hard-to-diagnose crashes.
// improvement: Set all `app_data` before calling `.run()`. If mutable state is needed, use `Arc<RwLock<T>>` or an actor.

use actix_web::{web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("hello") }))
    })
    .bind("127.0.0.1:8080")?;

    server = server.app_data(web::Data::new(42u32));

    server.run().await
}
