// [frensense]
// observation: Mutable access to a static or shared memory location is performed without a critical section, making it non-atomic and unsafe in interrupt context.
// impact: An interrupt occurring during the read-modify-write sequence can observe a torn value, leading to corrupted state, missed events, or undefined behavior on embedded targets.
// improvement: Disable interrupts around the access (e.g., `cortex_m::interrupt::free`) or use atomic types from `core::sync::atomic`.

#![no_std]

static mut SHARED_COUNTER: u32 = 0;

pub fn increment_shared() {
    unsafe {
        SHARED_COUNTER += 1;
    }
}
