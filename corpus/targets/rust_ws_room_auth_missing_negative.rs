use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use tokio::sync::broadcast;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms: Arc<Mutex<HashMap<String, (broadcast::Sender<String>, HashSet<String>)>>> = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:9003").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let rooms = rooms.clone();
        tokio::spawn(async move {
            let (mut write, mut read) = accept_async(stream).await.unwrap().split();
            let first = read.next().await.unwrap().unwrap();
            if let Message::Text(payload) = first {
                let parts: Vec<&str> = payload.splitn(2, ':').collect();
                let (room_name, token) = (parts[0], parts.get(1).unwrap_or(&""));
                let rx = {
                    let map = rooms.lock().await;
                    if let Some((_, allowed)) = map.get(room_name) {
                        // SAFE: Token-based access control prevents unauthorized room joins.
                        if !allowed.contains(*token) {
                            return;
                        }
                    }
                    if let Some((tx, _)) = map.get(room_name) {
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
