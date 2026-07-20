// [frensense]
// observation: A static mutable variable is accessed and mutated from multiple threads without synchronization, causing a data race.
// impact: Undefined behavior — concurrent reads and writes to the same memory location from multiple threads can cause crashes, data corruption, and security vulnerabilities.
// improvement: Use atomic types (AtomicU32, AtomicBool), Mutex, RwLock, or thread-local storage instead of static mut.

static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn get_counter() -> u32 {
    unsafe { COUNTER }
}

fn process_item(item_id: u32) {
    increment_counter();
    println!("Processed item {} (total: {})", item_id, get_counter());
}
