use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct CanaryServer {
    pub bind_addr: SocketAddr,
    inner: Arc<Mutex<CanaryState>>,
}

#[derive(Debug, Default)]
struct CanaryState {
    pending: HashMap<String, ()>,
    received: HashMap<String, ReceivedCallback>,
}

#[derive(Debug, Clone)]
pub struct ReceivedCallback {
    pub probe_id: String,
    pub source_ip: std::net::IpAddr,
    pub payload_snippet: String,
    pub received_at: std::time::SystemTime,
}

impl CanaryServer {
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self {
            bind_addr,
            inner: Arc::new(Mutex::new(CanaryState::default())),
        }
    }

    pub fn register_pending(&self, probe_id: &str) {
        self.inner
            .lock()
            .unwrap()
            .pending
            .insert(probe_id.to_string(), ());
    }

    pub fn check_received(&self, probe_id: &str) -> bool {
        self.inner
            .lock()
            .unwrap()
            .received
            .contains_key(probe_id)
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind(self.bind_addr)
            .await
            .expect("canary bind failed");
        let inner = self.inner.clone();
        tokio::spawn(async move {
            loop {
                if let Ok((mut stream, peer)) = listener.accept().await {
                    let inner = inner.clone();
                    tokio::spawn(async move {
                        let mut buf = vec![0u8; 4096];
                        let n = stream.read(&mut buf).await.unwrap_or(0);
                        let payload = String::from_utf8_lossy(&buf[..n]).to_string();

                        let mut state = inner.lock().unwrap();
                        let matched_id = state
                            .pending
                            .keys()
                            .find(|id| payload.contains(id.as_str()))
                            .cloned();
                        if let Some(id) = matched_id {
                            state.received.insert(
                                id.clone(),
                                ReceivedCallback {
                                    probe_id: id.clone(),
                                    source_ip: peer.ip(),
                                    payload_snippet: payload.chars().take(200).collect(),
                                    received_at: std::time::SystemTime::now(),
                                },
                            );
                            state.pending.remove(&id);
                        }
                    });
                }
            }
        });
    }
}
