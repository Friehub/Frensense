use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, HttpMessage};
use actix_web_actors::ws;
use actix_web::middleware::from_fn;

async fn auth_middleware(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    if let Some(val) = req.headers().get("Authorization") {
        if val == "Bearer valid-token" {
            // SAFE: Token validated before WebSocket upgrade.
            return ws::start(AuthedActor { user: "alice".into() }, &req, stream);
        }
    }
    Ok(HttpResponse::Unauthorized().body("unauthorized"))
}

struct AuthedActor {
    user: String,
}

impl actix::Actor for AuthedActor {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for AuthedActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(format!("{}: {}", self.user, text));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws", web::get().to(auth_middleware))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
