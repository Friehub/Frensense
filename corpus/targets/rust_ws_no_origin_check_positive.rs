// [frensense]
// observation: A WebSocket upgrade handler in a tokio-tungstenite server does not validate the `Origin` header before completing the handshake. Any third-party website can initiate a cross-origin WebSocket connection.
// impact: Cross-site WebSocket hijacking (CSWSH). An attacker's website can open a WebSocket to the server as the victim user, sending and receiving messages within the victim's session, leading to data exfiltration and unauthorized actions.
// improvement: Check the `Origin` header during the upgrade and reject connections from untrusted origins.

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    while let Ok((stream, peer)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (_, mut read) = ws_stream.split();
            while let Some(msg) = read.next().await {
                eprintln!("msg from {}: {:?}", peer, msg);
            }
        });
    }
    Ok(())
}
