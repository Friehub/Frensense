// SAFE: Conditional compilation ensures only one allocator is active per feature set.
#![no_std]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

pub struct SmallAlloc;
unsafe impl GlobalAlloc for SmallAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { core::ptr::null_mut() }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

pub struct BigAlloc;
unsafe impl GlobalAlloc for BigAlloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { core::ptr::null_mut() }
    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[cfg(feature = "small")]
#[global_allocator]
static A: SmallAlloc = SmallAlloc;

#[cfg(not(feature = "small"))]
#[global_allocator]
static A: BigAlloc = BigAlloc;
