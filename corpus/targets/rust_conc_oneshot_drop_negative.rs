use tokio::sync::oneshot;

pub async fn worker() -> Result<(), &'static str> {
    let (tx, rx) = oneshot::channel::<String>();
    tokio::spawn(async move {
        tx.send("result".into()).ok();
    });
    let msg = rx.await.map_err(|_| "sender dropped")?;
    println!("got: {}", msg);
    Ok(())
}
