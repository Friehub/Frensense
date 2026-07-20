// SAFE alternative: use safe wrapper with capacity tracking
struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    fn view(&self, offset: usize, count: usize) -> Option<&[u8]> {
        if offset + count > self.data.len() {
            return None;
        }
        Some(&self.data[offset..offset + count])
    }
}
