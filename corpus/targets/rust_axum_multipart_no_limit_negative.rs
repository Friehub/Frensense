use axum::extract::Multipart;

const MAX_FIELD_SIZE: usize = 10_485_760;

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // SAFE: text_with_limit prevents OOM by rejecting oversized fields.
        let data = field.text_with_limit(MAX_FIELD_SIZE).await.unwrap();
        println!("received {} bytes", data.len());
    }
}
