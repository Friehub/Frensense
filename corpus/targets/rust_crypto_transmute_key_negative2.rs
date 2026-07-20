// SAFE alternative: use byte-order-aware conversion with bytemuck
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct KeyBytes([u8; 32]);

fn derive_key(seed: &[u8; 32]) -> [u64; 4] {
    bytemuck::cast(*seed)
}
