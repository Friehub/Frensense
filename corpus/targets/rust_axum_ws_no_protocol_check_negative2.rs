use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::{IntoResponse, Response},
    http::HeaderMap,
};
use futures::StreamExt;

async fn ws_handler(ws: WebSocketUpgrade, headers: HeaderMap) -> Response {
    // SAFE: Whitelist-based protocol check on upgrade; reject unmatched protocols.
    let allowed = headers
        .get("sec-websocket-protocol")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|p| matches!(p, "chat" | "echo"));

    if !allowed {
        return "forbidden protocol".into_response();
    }

    ws.on_upgrade(handle_socket).into_response()
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            socket.send(Message::Text(text)).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/ws", axum::routing::get(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
