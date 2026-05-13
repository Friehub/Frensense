// tests/correctness/positive/ts_floating_promise.ts
// Rule: TS_FLOATING_PROMISE
async fn test() {
    fetch("https://api.example.com"); // No await
}
