// [frensense]
// observation: An Axum WebSocket upgrade handler accepts any `Sec-WebSocket-Protocol` header without validation. An attacker can use a non-standard protocol sub-protocol to confuse the server's message router or bypass per-protocol access controls.
// impact: Protocol confusion — the client can negotiate a sub-protocol that the server does not fully implement, leading to message misinterpretation or security control bypass.
// improvement: Validate the `Sec-WebSocket-Protocol` header against an allowlist on upgrade, and reject unknown protocols.

use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use futures::StreamExt;

async fn ws_handler(ws: WebSocketUpgrade) -> impl axum::response::IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            socket.send(Message::Text(text)).await.unwrap();
        }
    }
}
