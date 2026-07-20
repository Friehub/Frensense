use axum::extract::Multipart;
use bytes::Bytes;

const MAX_FIELD_SIZE: usize = 10_485_760;

async fn upload(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let mut total = 0usize;
        let mut chunks = Vec::new();
        while let Some(chunk) = field.chunk().await.unwrap() {
            total += chunk.len();
            // SAFE: Checking cumulative size per field prevents unbounded allocation.
            if total > MAX_FIELD_SIZE {
                return;
            }
            chunks.push(chunk);
        }
        let data: Vec<u8> = chunks.into_iter().flatten().collect();
        println!("received {} bytes", data.len());
    }
}
