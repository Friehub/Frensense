use tokio::sync::oneshot;

async fn demo() {
    let mut value = String::from("hello");
    let (tx1, mut rx1) = oneshot::channel::<()>();
    let (tx2, mut rx2) = oneshot::channel::<()>();

    // SAFE: Extracting the shared state after the select avoids cross-branch borrow conflicts.
    let suffix = tokio::select! {
        _ = &mut rx1 => " from rx1",
        _ = &mut rx2 => " from rx2",
    };
    value.push_str(suffix);

    println!("{value}");
}

#[tokio::main]
async fn main() {
    demo().await;
}
