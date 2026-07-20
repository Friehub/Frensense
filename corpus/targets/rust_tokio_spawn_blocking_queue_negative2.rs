use std::sync::mpsc;
use std::thread;

static POOL_SIZE: usize = 64;

fn blocking_pool() -> mpsc::Sender<Box<dyn FnOnce() + Send>> {
    let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
    for _ in 0..POOL_SIZE {
        let rx = rx.clone();
        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task();
            }
        });
    }
    tx
}

async fn handle_request(tx: mpsc::Sender<Box<dyn FnOnce() + Send>>, id: u32) {
    // SAFE: Dedicated thread pool with bounded threads prevents unbounded queue growth.
    tx.send(Box::new(move || {
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("processed {id}");
    }))
    .ok();
}

#[tokio::main]
async fn main() {
    let tx = blocking_pool();
    for i in 0..10_000u32 {
        tokio::spawn(handle_request(tx.clone(), i));
    }
}
