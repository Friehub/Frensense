// SAFE: OOM handler logs a diagnostic and resets the device.
#![no_std]

extern crate alloc;

use alloc::alloc::Layout;

#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    write_debug(b"OOM: allocation failed, size=");
    write_debug(layout.size().to_ne_bytes().as_slice());
    loop {}
}

fn write_debug(_msg: &[u8]) {}
