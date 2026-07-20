// SAFE alternative: use lock ordering struct or single global lock
use std::sync::Mutex;

struct AccountLock;
struct OrderLock;

fn transfer_money(accounts: &Mutex<AccountLock>, orders: &Mutex<OrderLock>) {
    let _a = accounts.lock().unwrap();
    let _o = orders.lock().unwrap();
}

fn process_batch(accounts: &Mutex<AccountLock>, orders: &Mutex<OrderLock>) {
    let _a = accounts.lock().unwrap();
    let _o = orders.lock().unwrap();
}
