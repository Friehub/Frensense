// [frensense]
// observation: Two or more `#[global_allocator]` static items are defined in the same binary, which the compiler rejects as ambiguous.
// impact: The binary fails to compile with error `` multiple `#[global_allocator]` ``. This blocks CI and produces a confusing error when feature flags cause duplicate allocator registrations.
// improvement: Ensure only one `#[global_allocator]` is present in the final binary, typically by gating each behind mutually exclusive features.

#![no_std]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};

pub struct AllocA;

unsafe impl GlobalAlloc for AllocA {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static A: AllocA = AllocA;

pub struct AllocB;

unsafe impl GlobalAlloc for AllocB {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static B: AllocB = AllocB;
