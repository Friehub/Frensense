use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::IntoResponse,
    http::HeaderMap,
};
use futures::StreamExt;

const ALLOWED_PROTOCOLS: &[&str] = &["chat", "echo"];

async fn ws_handler(ws: WebSocketUpgrade, headers: HeaderMap) -> impl IntoResponse {
    // SAFE: Validate Sec-WebSocket-Protocol against an allowlist before upgrading.
    if let Some(proto) = headers.get("sec-websocket-protocol") {
        if let Ok(proto_str) = proto.to_str() {
            if !ALLOWED_PROTOCOLS.contains(&proto_str) {
                return "unsupported protocol".into_response();
            }
        }
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
