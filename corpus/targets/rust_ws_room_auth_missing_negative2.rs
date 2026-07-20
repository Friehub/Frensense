use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
struct RoomAuth {
    members: HashMap<String, Vec<String>>,
}

impl RoomAuth {
    fn can_join(&self, room: &str, user: &str) -> bool {
        self.members.get(room).map_or(false, |users| users.contains(&user.to_string()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = Arc::new(RwLock::new(RoomAuth::default()));
    let rooms: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>> = Arc::new(RwLock::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:9003").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let auth = auth.clone();
        let rooms = rooms.clone();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let first = read.next().await.unwrap().unwrap();
            if let Message::Text(room_name) = first {
                let user = "alice";
                // SAFE: Explicit access check before joining the room.
                if !auth.read().await.can_join(&room_name, user) {
                    let _ = write.send(Message::Close(None)).await;
                    return;
                }
                let rx = {
                    let map = rooms.read().await;
                    if let Some(tx) = map.get(&room_name) {
                        tx.subscribe()
                    } else {
                        return;
                    }
                };
                while let Ok(msg) = rx.recv().await {
                    let _ = write.send(Message::Text(msg)).await;
                }
            }
        });
    }
    Ok(())
}
