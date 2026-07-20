// SAFE: Uses `ouroboros` crate for safe self-referential struct.
use ouroboros::self_referencing;

#[self_referencing]
pub struct BufferView {
    buf: Vec<u8>,
    #[borrows(buf)]
    view: &'this [u8],
}

impl BufferView {
    pub fn new() -> Self {
        BufferViewBuilder {
            buf: vec![0u8; 1024],
            view_builder: |buf: &Vec<u8>| &buf[..],
        }
        .build()
    }
}
