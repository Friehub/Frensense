// [frensense]
// observation: `tokio::select!` borrows the same mutable data across multiple branches. The macro expands each branch into a separate future, and the borrow checker cannot prove the borrows are disjoint, causing a compilation error.
// impact: The code does not compile. Developers may resort to `unsafe` or `RefCell` to work around the borrow conflict, introducing runtime borrow-checking overhead or undefined behavior.
// improvement: Restructure to avoid borrowing the same data in multiple branches, e.g., by extracting the shared value before the select or using `Arc<Mutex<T>>`.

use tokio::sync::oneshot;

async fn demo() {
    let mut value = String::from("hello");
    let (tx1, mut rx1) = oneshot::channel::<()>();
    let (tx2, mut rx2) = oneshot::channel::<()>();

    tokio::select! {
        _ = &mut rx1 => {
            value.push_str(" from rx1");
        }
        _ = &mut rx2 => {
            value.push_str(" from rx2");
        }
    }

    println!("{value}");
}

#[tokio::main]
async fn main() {
    demo().await;
}
