// SAFE: Uses a fixed-size bump allocator that cannot OOM; no `alloc_error_handler` needed.
#![no_std]

use core::mem::MaybeUninit;

pub struct BumpAlloc<const N: usize> {
    pool: MaybeUninit<[u8; N]>,
    offset: core::sync::atomic::AtomicUsize,
}

impl<const N: usize> BumpAlloc<N> {
    pub const fn new() -> Self {
        Self {
            pool: MaybeUninit::uninit(),
            offset: core::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn alloc(&self, size: usize) -> Option<*mut u8> {
        let align = core::mem::align_of::<u8>();
        let off = self.offset.fetch_add(size, core::sync::atomic::Ordering::Relaxed);
        let base = self.pool.as_ptr() as *mut u8;
        let ptr = unsafe { base.add(off) };
        if off + size > N {
            return None;
        }
        Some(ptr)
    }
}
