// SAFE: Uses cortex_m's interrupt-free critical section for the mutation.
#![no_std]

static mut SHARED_COUNTER: u32 = 0;

pub fn increment_shared() {
    cortex_m::interrupt::free(|_| unsafe {
        SHARED_COUNTER += 1;
    });
}
