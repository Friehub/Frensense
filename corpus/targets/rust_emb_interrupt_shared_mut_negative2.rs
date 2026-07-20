// SAFE: Uses volatile reads/writes so the compiler cannot optimize away the access.
#![no_std]

use core::ptr::{read_volatile, write_volatile};

static mut INTERRUPT_FLAG: bool = false;

#[interrupt]
fn TIM2() {
    unsafe { write_volatile(&mut INTERRUPT_FLAG, true); }
}

pub fn poll_flag() -> bool {
    unsafe { read_volatile(&INTERRUPT_FLAG) }
}
