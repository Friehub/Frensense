// SAFE: Watchdog fed from a dedicated timer interrupt for deterministic timing.
#![no_std]

use cortex_m::peripheral::{WDT, TIM};

pub fn init_watchdog(wdt: &mut WDT, tim: &mut TIM) {
    wdt.start(2_000_000u32);
    tim.set_compare(500_000u32);
    tim.enable_interrupt();
}

fn watchdog_safe() {
    // Called from TIM interrupt
    let wdt = unsafe { &mut *cortex_m::peripheral::WDT::ptr() };
    wdt.feed();
}
