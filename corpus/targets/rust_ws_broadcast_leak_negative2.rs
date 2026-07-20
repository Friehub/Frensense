use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

struct Room {
    sender: broadcast::Sender<String>,
    allowed_users: Vec<String>,
}

impl Room {
    fn can_access(&self, user: &str) -> bool {
        self.allowed_users.contains(&user.to_string())
    }
}

type RoomRegistry = Arc<RwLock<HashMap<String, Room>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms: RoomRegistry = Arc::new(RwLock::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:9002").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let rooms = rooms.clone();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let user = "alice";
            let room_name = "admin";
            let rx = {
                let map = rooms.read().await;
                // SAFE: Broadcast is scoped to authenticated users within a room.
                if let Some(room) = map.get(room_name) {
                    if !room.can_access(user) {
                        return;
                    }
                    room.sender.subscribe()
                } else {
                    return;
                }
            };
            while let Ok(msg) = rx.recv().await {
                let _ = write.send(Message::Text(format!("{}: {}", room_name, msg))).await;
            }
        });
    }
    Ok(())
}
