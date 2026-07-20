// SAFE: Critical section disabled interrupts during the mutation.
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

static SHARED_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn increment_shared() {
    SHARED_COUNTER.fetch_add(1, Ordering::SeqCst);
}
