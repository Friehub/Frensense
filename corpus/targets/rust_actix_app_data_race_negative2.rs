use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_state = Arc::new(RwLock::new(42u32));
    // SAFE: Mutable state uses Arc<RwLock> for safe interior mutability across workers.
    HttpServer::new(move || {
        let state = shared_state.clone();
        App::new()
            .app_data(web::Data::new(state))
            .route("/", web::get().to(move || {
                let s = shared_state.clone();
                async move {
                    let val = *s.read().await;
                    HttpResponse::Ok().body(format!("{}", val))
                }
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
