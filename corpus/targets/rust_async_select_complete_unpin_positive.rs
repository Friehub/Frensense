// [frensene]
// observation: A non-`Unpin` future is used inside `tokio::select!` without being pinned. `tokio::select!` requires all branches to be `Unpin`; non-`Unpin` futures may move and corrupt internal state.
// impact: Undefined behavior — the future's internal self-referential pointers become invalid after movement, leading to memory corruption or panics.
// improvement: Pin the future before using in `select!` with `pin_mut!` or `Box::pin`.

use tokio::select;

pub async fn race() {
    let fut = async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        42
    };
    select! {
        v = fut => println!("{}", v),
    }
}
