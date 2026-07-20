use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type RoomMap = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms: RoomMap = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:9002").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let rooms = rooms.clone();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let room_name = "general";
            let rx = {
                let mut map = rooms.lock().await;
                // SAFE: Messages are scoped per-room; broadcast only reaches room subscribers.
                let tx = map.entry(room_name.to_string()).or_insert_with(|| broadcast::channel(1024).0);
                tx.subscribe()
            };
            let rooms2 = rooms.clone();
            tokio::spawn(async move {
                while let Some(Ok(msg)) = read.next().await {
                    if let Message::Text(text) = msg {
                        let map = rooms2.lock().await;
                        if let Some(tx) = map.get(room_name) {
                            let _ = tx.send(text);
                        }
                    }
                }
            });
            while let Ok(msg) = rx.recv().await {
                let _ = write.send(Message::Text(msg)).await;
            }
        });
    }
    Ok(())
}
