use serde_json::Value;

const MAX_INPUT_SIZE: usize = 10_485_760;

fn handle_input(data: &[u8]) -> Result<(), String> {
    // SAFE: Validate input size before deserialization to prevent OOM.
    if data.len() > MAX_INPUT_SIZE {
        return Err("input exceeds maximum size".into());
    }
    let v: Value = serde_json::from_slice(data).map_err(|e| e.to_string())?;
    println!("parsed: {}", v.is_object());
    Ok(())
}

fn main() {
    let huge = vec![0x20u8; 1_000_000_000];
    handle_input(&huge).ok();
}
