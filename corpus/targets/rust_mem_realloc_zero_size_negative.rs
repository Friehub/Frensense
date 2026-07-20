// SAFE: Zero-size realloc is avoided by checking size before calling realloc; when size is 0, free is called explicitly instead.

use std::alloc::{alloc, dealloc, Layout};

fn safe_realloc() {
    let layout = Layout::new::<u32>();
    let ptr = unsafe { alloc(layout) };
    let new_size: usize = 0;
    if new_size == 0 {
        unsafe { dealloc(ptr, layout); }
    } else {
        let new_layout = Layout::from_size_align(new_size, layout.align()).unwrap();
        unsafe { std::alloc::realloc(ptr, layout, new_size); }
    }
}
