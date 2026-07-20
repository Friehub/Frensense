// SAFE: Uses an index instead of a pointer into the struct, avoiding self-references.
pub struct BufferView {
    buf: Vec<u8>,
    offset: usize,
    length: usize,
}

impl BufferView {
    pub fn new() -> Self {
        let buf = vec![0u8; 1024];
        BufferView { offset: 0, length: buf.len(), buf }
    }

    pub fn get_view(&self) -> &[u8] {
        &self.buf[self.offset..self.offset + self.length]
    }
}
