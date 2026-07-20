// SAFE: Length is capped to the allocated region via a known-good layout.
pub struct SafeBuffer {
    data: Vec<u8>,
}

impl SafeBuffer {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}
