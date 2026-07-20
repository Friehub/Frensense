// [frensense]
// observation: realloc(ptr, 0) or realloc_in_place with size 0 is called. The C standard says realloc(ptr, 0) is implementation-defined — it may free the pointer (like free) or return NULL, causing a double-free or use-after-free.
// impact: Platform-dependent undefined behavior: on glibc, realloc(ptr, 0) frees the pointer; on other platforms it returns NULL. This can cause double-free, use-after-free, or memory corruption.
// improvement: Check for zero size before calling realloc; use free() explicitly when size is 0.

use std::alloc::{alloc, dealloc, realloc, Layout};

fn unsafe_realloc_zero() {
    let layout = Layout::new::<u32>();
    let ptr = unsafe { alloc(layout) };
    unsafe {
        let _new_ptr = realloc(ptr, layout, 0);
    }
}

fn unsafe_realloc_zero_in_place() {
    let layout = Layout::from_size_align(64, 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    unsafe {
        let _ = std::alloc::realloc(ptr, layout, 0);
    }
}
