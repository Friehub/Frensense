// SAFE: Watchdog is fed every iteration of the main loop.
#![no_std]

use cortex_m::peripheral::WDT;

pub fn init_watchdog(wdt: &mut WDT) {
    wdt.start(1_000_000u32);
}

pub fn main_loop(mut wdt: WDT) -> ! {
    loop {
        process_sensors();
        wdt.feed();
    }
}

fn process_sensors() {}
