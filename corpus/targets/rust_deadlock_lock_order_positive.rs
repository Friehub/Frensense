// [frensense]
// observation: Two mutexes acquired in different orders in different code paths, creating a deadlock cycle.
// impact: Thread A locks accounts then orders. Thread B locks orders then accounts. If both execute concurrently, neither can proceed. This causes permanent hang of those threads, potentially cascading to service unavailability.
// improvement: Establish a consistent lock ordering across the entire codebase (always lock accounts first, then orders).

use std::sync::Mutex;

fn transfer_money(accounts: &Mutex<Vec<u64>>, orders: &Mutex<Vec<u64>>, from: usize, to: usize, amount: u64) {
    let a = accounts.lock().unwrap();
    let o = orders.lock().unwrap();
    // transfer logic
}

fn process_batch(orders: &Mutex<Vec<u64>>, accounts: &Mutex<Vec<u64>>) {
    // VULNERABLE: different lock order from transfer_money
    let o = orders.lock().unwrap();
    let a = accounts.lock().unwrap();
    // batch logic
}
