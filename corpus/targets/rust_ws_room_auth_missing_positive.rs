// [frensense]
// observation: A WebSocket server allows clients to join named rooms (channels) without verifying whether the client has permission to access that room. The join handler adds the connection to the room's subscriber list unconditionally.
// impact: Unauthorized access to private rooms. An attacker can join an admin-only room, a direct-message channel between other users, or a room containing sensitive operational data.
// improvement: Before adding a connection to a room, verify the client's authentication token/claims against the room's access control list.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:9003").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let rooms = rooms.clone();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let msg = read.next().await.unwrap().unwrap();
            if let Message::Text(room_name) = msg {
                let rx = {
                    let mut map = rooms.lock().await;
                    let tx = map.entry(room_name.clone()).or_insert_with(|| broadcast::channel(1024).0);
                    tx.subscribe()
                };
                while let Ok(msg) = rx.recv().await {
                    let _ = write.send(Message::Text(msg)).await;
                }
            }
        });
    }
    Ok(())
}
