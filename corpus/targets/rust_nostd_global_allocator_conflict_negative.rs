// SAFE: Only one `#[global_allocator]` is defined.
#![no_std]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

pub struct MyAlloc;

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: MyAlloc = MyAlloc;
