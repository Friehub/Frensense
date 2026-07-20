// [frensense]
// observation: An Axum multipart form field is read entirely into memory without a size limit per field. An attacker can upload a single massive field to exhaust server memory.
// impact: Unbounded per-field memory allocation leads to OOM denial of service. The server becomes unresponsive under multipart upload attacks.
// improvement: Use `.text_with_limit()` or `.bytes_with_limit()` on each field, or stream chunks with a length check.

use axum::extract::Multipart;

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.text().await.unwrap();
        println!("received {} bytes", data.len());
    }
}
