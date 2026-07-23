// SAFE: Uses a custom authentication middleware on the admin scope
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, middleware, body::MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, Service};
use std::future::{ready, Ready};
use std::task::{Context, Poll};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = S::Future;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authed = req.headers().get("Authorization").and_then(|v| v.to_str().ok());
        if authed.is_none() {
            let (req, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized().finish();
            let svc_resp = ServiceResponse::new(req, response);
            return async { Ok(svc_resp.map_into_boxed_body()) }.into();
        }
        self.service.call(req)
    }
}

async fn admin_dashboard() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"users": ["alice", "bob"]}))
}

async fn delete_user(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({"deleted": user_id}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/admin")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(admin_dashboard))
                    .route("/users/{id}", web::delete().to(delete_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
