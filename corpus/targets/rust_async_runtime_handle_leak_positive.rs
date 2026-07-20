// [frensense]
// observation: Runtime::block_on is called inside an existing async context, which can cause a panic if the runtime handle leaks across tasks or is called recursively.
// impact: Calling block_on within an async context on the same runtime causes a panic ("Cannot block the current thread from within a runtime"), crashing the application.
// improvement: Use .await instead of block_on in async code, or spawn a dedicated thread for blocking operations.

use tokio::runtime::Runtime;

async fn fetch_data() -> Result<String, reqwest::Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        reqwest::get("https://example.com").await?.text().await
    })
}

async fn handler() -> String {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(compute_heavy());
    format!("result: {}", result)
}

fn sync_wrapper() -> String {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let inner_rt = Runtime::new().unwrap();
        inner_rt.block_on(fetch_other())
    })
}
