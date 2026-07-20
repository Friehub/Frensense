// SAFE: consistent lock order — accounts before orders everywhere
use std::sync::Mutex;

fn transfer_money(accounts: &Mutex<Vec<u64>>, orders: &Mutex<Vec<u64>>, from: usize, to: usize, amount: u64) {
    let a = accounts.lock().unwrap();
    let o = orders.lock().unwrap();
}

fn process_batch(accounts: &Mutex<Vec<u64>>, orders: &Mutex<Vec<u64>>) {
    let a = accounts.lock().unwrap();
    let o = orders.lock().unwrap();
}
