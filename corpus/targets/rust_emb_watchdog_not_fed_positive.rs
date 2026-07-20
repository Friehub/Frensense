// [frensense]
// observation: A watchdog timer is configured but never fed (refreshed) in the main loop, so it will expire and reset the system after the timeout period.
// impact: The device resets periodically for no reason, causing downtime, data loss, or missed real-time deadlines in production.
// improvement: Call the watchdog feed function periodically in the main loop or in a high-priority timer interrupt.

#![no_std]

use cortex_m::peripheral::WDT;

pub fn init_watchdog(wdt: &mut WDT) {
    wdt.start(1_000_000u32);
}

pub fn main_loop() -> ! {
    loop {
        process_sensors();
        // WDT never fed — system will reset!
    }
}

fn process_sensors() {}
