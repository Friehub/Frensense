use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

async fn demo() {
    let value = Arc::new(Mutex::new(String::from("hello")));
    let (tx1, mut rx1) = oneshot::channel::<()>();
    let (tx2, mut rx2) = oneshot::channel::<()>();

    tokio::select! {
        _ = &mut rx1 => {
            // SAFE: Arc<Mutex<T>> allows shared mutable access across branches without borrow conflicts.
            value.lock().await.push_str(" from rx1");
        }
        _ = &mut rx2 => {
            value.lock().await.push_str(" from rx2");
        }
    }

    println!("{}", value.lock().await);
}

#[tokio::main]
async fn main() {
    demo().await;
}
