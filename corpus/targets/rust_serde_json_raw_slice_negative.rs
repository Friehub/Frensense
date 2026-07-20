use serde_json::Value;
use std::io::Read;

const MAX_INPUT_SIZE: u64 = 10_485_760;

struct SizeLimitedReader<R> {
    inner: R,
    limit: u64,
    total: u64,
}

impl<R: Read> Read for SizeLimitedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.total += n as u64;
        if self.total > self.limit {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "input too large"));
        }
        Ok(n)
    }
}

fn handle_input(data: &[u8]) -> Result<(), serde_json::Error> {
    // SAFE: from_reader with a size limit prevents OOM from untrusted large input.
    let limited = SizeLimitedReader { inner: data, limit: MAX_INPUT_SIZE, total: 0 };
    let v: Value = serde_json::Deserializer::from_reader(limited).into_iter().next().unwrap()?;
    println!("parsed: {}", v.is_object());
    Ok(())
}

fn main() {
    let huge = vec![0x20u8; 1_000_000_000];
    handle_input(&huge).ok();
}
