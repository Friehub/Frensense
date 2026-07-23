// SAFE: Panic handler logs a message and aborts the device.
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        // Write panic location to a debug UART
        write_debug(b"PANIC at ");
        write_debug(loc.file().as_bytes());
    }
    loop {}
}

fn write_debug(_msg: &[u8]) {}
