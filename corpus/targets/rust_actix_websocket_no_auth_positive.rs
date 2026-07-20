// [frensense]
// observation: An Actix WebSocket actor is created directly from an HTTP request handler without verifying the caller's authentication. The `HttpRequest` (or `Identity`) is not checked for a valid session or token before upgrading to a WebSocket connection.
// impact: Unauthenticated users can establish WebSocket connections and exchange messages with the server, bypassing application-level authentication. Attackers can eavesdrop on real-time data or inject malicious messages.
// improvement: Validate the user's authentication (e.g., session cookie, JWT, or API key) in the HTTP upgrade handler before calling `ws::start()`.

use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use actix_web_actors::ws;

async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(MyActor, &req, stream)
}

struct MyActor;

impl actix::Actor for MyActor {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyActor {
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
