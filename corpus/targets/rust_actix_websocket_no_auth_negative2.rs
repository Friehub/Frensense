use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix_web::cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let token = req.cookie("session")
        .and_then(|c: Cookie| decode::<serde_json::Value>(c.value(), &DecodingKey::from_secret(b"secret"), &Validation::new(Algorithm::HS256)).ok());
    // SAFE: Session JWT validated before WebSocket upgrade prevents unauthenticated connections.
    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().body("unauthorized"));
    }
    ws::start(AuthedActor, &req, stream)
}

struct AuthedActor;

impl actix::Actor for AuthedActor {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for AuthedActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text);
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws", web::get().to(ws_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
