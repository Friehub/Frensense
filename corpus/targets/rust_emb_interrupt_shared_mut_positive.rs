// [frensense]
// observation: An interrupt handler writes to a static variable that is also read by non-interrupt code, without using `core::ptr::read_volatile` or `write_volatile`, so the compiler may optimize away the access or reorder it.
// impact: The non-interrupt code may read a stale or cached value, causing missed events, incorrect state, or data races that manifest as heisenbugs.
// improvement: Declare the variable as `volatile` (e.g., use `core::ptr::read_volatile`/`write_volatile` or wrap in a `VolatileCell`).

#![no_std]

static mut INTERRUPT_FLAG: bool = false;

#[interrupt]
fn TIM2() {
    unsafe {
        INTERRUPT_FLAG = true;
    }
}

pub fn poll_flag() -> bool {
    unsafe { INTERRUPT_FLAG }
}
