// [frensense]
// observation: `serde_json::from_slice` is called on an untrusted byte slice whose length is controlled by the caller. Large inputs cause the deserializer to allocate significant memory (e.g., for string values, arrays, or nested objects) proportional to input size.
// impact: An attacker can send a multi-gigabyte JSON payload to exhaust server memory via OOM. The `from_slice` API eagerly parses the entire input, unlike streaming parsers.
// improvement: Limit input size before deserialization, or use `serde_json::Deserializer::from_reader` with a size-bounded reader.

use serde_json::Value;

fn handle_input(data: &[u8]) -> Result<(), serde_json::Error> {
    let v: Value = serde_json::from_slice(data)?;
    println!("parsed: {}", v.is_object());
    Ok(())
}

fn main() {
    let huge = vec![0x20u8; 1_000_000_000];
    handle_input(&huge).ok();
}
