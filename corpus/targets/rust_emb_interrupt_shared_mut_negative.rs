// SAFE: Shared flag uses `AtomicBool` which provides correct semantics without volatile.
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

static INTERRUPT_FLAG: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn TIM2() {
    INTERRUPT_FLAG.store(true, Ordering::SeqCst);
}

pub fn poll_flag() -> bool {
    INTERRUPT_FLAG.load(Ordering::SeqCst)
}
